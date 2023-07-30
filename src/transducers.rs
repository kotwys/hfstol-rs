use alloc::{
    boxed::Box,
    vec::Vec, vec,
    string::String,
};

mod header;
mod alphabet;
mod cursor;
mod transitions;
mod weighted;

use self::header::Header;
use self::alphabet::Alphabet;
use self::transitions::{TransitionIndex, WeightedTransition};
use crate::trie::Trie;
use crate::parser_utils::parse_to_vec_n;

pub type KeyTable = Vec<String>;
pub type Symbol = u16;
pub type TransitionTableIndex = u32;
pub type Weight = f32;

pub const NO_SYMBOL_NUMBER: Symbol = Symbol::MAX;
pub const EPSILON: Symbol = 0;
pub const NO_TABLE_INDEX: TransitionTableIndex = TransitionTableIndex::MAX;
pub const TRANSITION_TARGET_TABLE_START: TransitionTableIndex = 1 << 31;

#[derive(Debug)]
pub enum Error {
    HeaderParsingError,
    SymbolTableParsingError,
    TableParsingError,
    TokenizationError,
    DecodingError,
    UnsupportedTransducerError,
    SyncError,
}

/// Trait all transducers should implement.
///
/// Run [`Transducer::lookup()`] with a `&str` to get analyses for the string.
pub trait Transducer {
    fn key_table(&self) -> &KeyTable;
    fn input_letters(&self) -> &Trie<Symbol>;
    
    /// Peforms a lookup of pre-encoded string.
    ///
    /// See [`Transducer::lookup()`].
    fn lookup_encoded(&self, input: &[Symbol]) -> Result<Vec<(Vec<Symbol>, Weight)>, Error>;

    fn tokenize(&self, mut input: &str) -> Result<Vec<Symbol>, Error> {
        let mut res = vec![];
        let trie = self.input_letters();
        while !input.is_empty() {
            match trie.get(input) {
                (Some(sym), rest) => {
                    res.push(*sym);
                    input = rest;                
                },
                _ => return Err(Error::TokenizationError),
            }
        }
        Ok(res)
    }

    /// Decodes an encoded string, writing to the given [`&mut String`].
    fn decode_to(&self, mut input: &[Symbol], result: &mut String) -> Result<(), Error> {
        let kt = self.key_table();
        while !input.is_empty() {
            result.push_str(
                kt.get(input[0] as usize).ok_or(Error::DecodingError)?
            );
            input = &input[1..];
        }
        Ok(())
    }

    /// Decodes an encoded string.
    fn decode(&self, input: &[Symbol]) -> Result<String, Error> {
        let mut result = String::new();
        self.decode_to(input, &mut result)?;
        Ok(result)
    }
    
    /// Performs a lookup of the given string.
    ///
    /// Returns a vector of tuples of resulting strings and weights.  If the
    /// transducer is unweighted, all the weights are 0.0.
    ///
    /// See [`Transducer::lookup_encoded()`] for looking up pre-encoded strings.
    fn lookup(&self, input: &str) -> Result<Vec<(String, Weight)>, Error> {
        let tokens = self.tokenize(input)?;
        self.lookup_encoded(&tokens)
            .and_then(|analyses| {
                analyses.into_iter()
                    .try_fold(vec![], |mut v, analysis| {
                        v.push((self.decode(&analysis.0)?, analysis.1));
                        Ok(v)
                    })
            })
    }

    /// Sets a maximum count of analyses performed.
    ///
    /// There might be a bit more analyses than `count` performed and added to
    /// the results vector of `Transducer::lookup()`.
    ///
    /// `count == 0` means no limit is imposed.
    fn set_max_analyses(&mut self, count: usize);
}

/// Reads a binary transducer and initializes an appropriate implementation.
///
/// Returns [`Error::UnsupportedTransducerError`] if no transducer could be
/// created.
pub fn read_transducer(input: &[u8]) -> Result<Box<dyn Transducer + Sync + Send>, Error> {
    let (input, header) = Header::parse(input)
        .map_err(|_| Error::HeaderParsingError)?;
    let (input, alphabet) = Alphabet::parse(input, header.number_of_symbols())
        .map_err(|_| Error::SymbolTableParsingError)?;
    let (input, index) = parse_to_vec_n(
        header.size_of_transition_index_table() as usize,
        TransitionIndex::parse,
    )(input).map_err(|_| Error::TableParsingError)?;

    // TODO: Other types of transducers
    match (header.weighted(), alphabet.state_size() > 0) {
        (true, false) => {
            let (_input, transitions) = parse_to_vec_n(
                header.size_of_transition_target_table() as usize,
                WeightedTransition::parse
            )(input).map_err(|_| Error::TableParsingError)?;
            Ok(Box::new(
                self::weighted::WeightedTransducer::new(
                    header, alphabet, index, transitions,
                )
            ))
        },
        _ => Err(Error::UnsupportedTransducerError),
    }
}
