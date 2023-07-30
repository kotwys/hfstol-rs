use alloc::{
    vec::Vec,
    string::String,
};
use nom::{
    IResult,
    bytes::complete::{tag, take_till},
    sequence::terminated,
};

use super::{KeyTable, Symbol};
use crate::trie::Trie;

pub struct Alphabet {
    key_table: KeyTable,
    state_size: Symbol,
}

impl Alphabet {
    fn parse_flag_diacritic(input: &[u8]) -> bool {
        // TODO: Parse flag diacritics properly
        input.len() > 4 &&
            input[0] == b'@' &&
            input[input.len()-1] == b'@'
            && input[2] == b'.'
    }
    
    pub fn parse(input: &[u8], number_of_symbols: Symbol) -> IResult<&[u8], Alphabet> {
        let mut key_table = Vec::with_capacity(number_of_symbols as usize);
        let mut state_size = 0u16;
        let input = (0..number_of_symbols)
            .try_fold(input, |input, _| {
                let (input, s) = terminated(take_till(|b| b == 0), tag(b"\0"))(input)?;
                if Alphabet::parse_flag_diacritic(s) {
                    state_size += 1;
                } else {
                    key_table.push(unsafe {
                        String::from_utf8_unchecked(s.to_vec())
                    });
                }
                Ok(input)
            })?;
        key_table[0] = String::new();
        Ok((input, Alphabet {
            key_table,
            state_size,
        }))
    }

    pub fn to_trie(&self, number_of_input_symbols: Symbol) -> Trie<Symbol> {
        let mut trie = Trie::new(None);
        for k in 1..number_of_input_symbols {
            trie.insert(&self.key_table[k as usize], k);
        }
        trie
    }
}

impl Alphabet {
    pub fn key_table(&self) -> &KeyTable {
        &self.key_table
    }

    pub fn state_size(&self) -> Symbol {
        self.state_size
    }
}
