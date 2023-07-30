use nom::{
    IResult,
    bytes::complete::{tag, take},
    number::complete::{le_u16, le_u32},
    sequence::terminated,
    combinator::opt,
};

use crate::parser_utils::{int_bool, parse_to_struct};
use super::{Symbol, TransitionTableIndex};

#[allow(dead_code)]
pub struct Header {
    number_of_input_symbols: Symbol,
    number_of_symbols: Symbol,
    size_of_transition_index_table: TransitionTableIndex,
    size_of_transition_target_table: TransitionTableIndex,

    number_of_states: u32,
    number_of_transitions: u32,

    weighted: bool,
    deterministic: bool,
    input_deterministic: bool,
    minimized: bool,
    cyclic: bool,
    has_epsilon_epsilon_transitions: bool,
    has_input_epsilon_transitions: bool,
    has_input_epsilon_cycles: bool,
    has_unweighted_input_epsilon_cycles: bool,
}

impl Header {
    fn skip_hfst3_header(input: &[u8]) -> IResult<&[u8], ()> {
        let (input, _) = tag(b"HFST\0")(input)?;
        let (input, length) = terminated(le_u16, tag(b"\0"))(input)?;
        let (input, _) = terminated(take(length-1), tag(b"\0"))(input)?;
        // Do nothing more for now
        Ok((input, ()))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Header> {
        let (input, _) = opt(Header::skip_hfst3_header)(input)?;
        parse_to_struct!(input, Header {
            number_of_input_symbols: le_u16,
            number_of_symbols: le_u16,
            size_of_transition_index_table: le_u32,
            size_of_transition_target_table: le_u32,
            number_of_states: le_u32,
            number_of_transitions: le_u32,
            weighted: int_bool,
            deterministic: int_bool,
            input_deterministic: int_bool,
            minimized: int_bool,
            cyclic: int_bool,
            has_epsilon_epsilon_transitions: int_bool,
            has_input_epsilon_transitions: int_bool,
            has_input_epsilon_cycles: int_bool,
            has_unweighted_input_epsilon_cycles: int_bool,
        })
    }
}

#[allow(dead_code)]
impl Header {
    pub fn number_of_symbols(&self) -> Symbol {
        self.number_of_symbols
    }
    pub fn number_of_input_symbols(&self) -> Symbol {
        self.number_of_input_symbols
    }
    pub fn size_of_transition_index_table(&self) -> TransitionTableIndex {
        self.size_of_transition_index_table
    }
    pub fn size_of_transition_target_table(&self) -> TransitionTableIndex {
        self.size_of_transition_target_table
    }
    pub fn weighted(&self) -> bool {
        self.weighted
    }
    pub fn deterministic(&self) -> bool {
        self.deterministic
    }
    pub fn input_deterministic(&self) -> bool {
        self.input_deterministic
    }
    pub fn minimized(&self) -> bool {
        self.minimized
    }
    pub fn cyclic(&self) -> bool {
        self.cyclic
    }
    pub fn has_epsilon_epsilon_transitions(&self) -> bool {
        self.has_epsilon_epsilon_transitions
    }
    pub fn has_input_epsilon_transitions(&self) -> bool {
        self.has_input_epsilon_transitions
    }
    pub fn has_input_epsilon_cycles(&self) -> bool {
        self.has_input_epsilon_cycles
    }
    pub fn has_unweighted_input_epsilon_cycles(&self) -> bool {
        self.has_unweighted_input_epsilon_cycles
    }
}
