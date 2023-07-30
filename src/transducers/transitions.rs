use nom::{
    IResult,
    number::complete::{le_u16, le_u32, le_f32},
};

use super::{
    Symbol, TransitionTableIndex, Weight,
    NO_SYMBOL_NUMBER, NO_TABLE_INDEX
};
use crate::parser_utils::parse_to_struct;

#[derive(Debug)]
pub struct TransitionIndex {
    symbol: Symbol,
    target: TransitionTableIndex,
}

impl TransitionIndex {
    pub fn parse(input: &[u8]) -> IResult<&[u8], TransitionIndex> {
        parse_to_struct!(input, TransitionIndex {
            symbol: le_u16,
            target: le_u32,
        })
    }

    pub fn is_final(&self) -> bool {
        self.symbol == NO_SYMBOL_NUMBER && self.target != NO_TABLE_INDEX
    }

    pub fn weight(&self) -> Weight {
        self.target as Weight
    }

    pub fn target(&self) -> TransitionTableIndex {
        self.target
    }
    pub fn symbol(&self) -> Symbol {
        self.symbol
    }
}

#[derive(Debug)]
pub struct WeightedTransition {
    input: Symbol,
    output: Symbol,
    target: TransitionTableIndex,
    weight: Weight,
}

impl WeightedTransition {
    pub fn parse(input_: &[u8]) -> IResult<&[u8], WeightedTransition> {
        parse_to_struct!(input_, WeightedTransition {
            input: le_u16,
            output: le_u16,
            target: le_u32,
            weight: le_f32,
        })
    }

    pub fn is_final(&self) -> bool {
        self.input == NO_SYMBOL_NUMBER
            && self.output == NO_SYMBOL_NUMBER
            && self.target == 1
    }

    pub fn input(&self) -> Symbol {
        self.input
    }
    pub fn output(&self) -> Symbol {
        self.output
    }
    pub fn target(&self) -> TransitionTableIndex {
        self.target
    }
    pub fn weight(&self) -> Weight {
        self.weight
    }
}

impl Default for WeightedTransition {
    fn default() -> Self {
        Self {
            input: NO_SYMBOL_NUMBER,
            output: NO_SYMBOL_NUMBER,
            target: NO_TABLE_INDEX,
            weight: NO_TABLE_INDEX as f32,
        }
    }
}
