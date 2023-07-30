use alloc::{
    vec::Vec,
    sync::Arc,
};
use crate::mutex::Mutex;
use super::{
    Transducer,
    Error,
    Header, Alphabet,
    KeyTable, Trie,
    TransitionIndex,
    TransitionTableIndex,
    Symbol, Weight,
    transitions::WeightedTransition,
    cursor::Cursor,
    TRANSITION_TARGET_TABLE_START, EPSILON,
};

pub struct WeightedTransducer {
    // header: Header,
    alphabet: Alphabet,
    input_letters: Trie<Symbol>,
    index: Vec<TransitionIndex>,
    transitions: Vec<WeightedTransition>,
    max_analyses: usize,
}

impl WeightedTransducer {
    pub fn new(
        header: Header,
        alphabet: Alphabet,
        index: Vec<TransitionIndex>,
        transitions: Vec<WeightedTransition>,
    ) -> Self {
        let input_letters = alphabet.to_trie(header.number_of_input_symbols());
        WeightedTransducer {
            // header,
            alphabet,
            input_letters,
            index,
            transitions,
            max_analyses: 0,
        }
    }

    fn try_transitions(
        &self,
        input_string: &[Symbol],
        cursor: Arc<Mutex<Cursor>>,
        total_analyses: Arc<Mutex<Vec<(Vec<Symbol>, Weight)>>>,
        mut index: TransitionTableIndex,
        expect: Symbol
    ) {
        while let Some(tr) = self.transitions.get(index as usize).filter(|tr| tr.input() == expect) {
            {
                let mut cursor = cursor.lock().unwrap();
                cursor.add_weight(tr.weight());
                cursor.push(tr.output());
            }
            self.analyze(
                if expect == EPSILON {
                    input_string                    
                } else {
                    &input_string[1..]
                },
                Arc::clone(&cursor),
                Arc::clone(&total_analyses),
                tr.target(),
            );
            {
                let mut cursor = cursor.lock().unwrap();
                cursor.take_weight(tr.weight());
                cursor.retract(1);
            }
            index += 1;
        }
    }
        
    fn analyze(
        &self,
        input_string: &[Symbol],
        cursor: Arc<Mutex<Cursor>>,
        total_analyses: Arc<Mutex<Vec<(Vec<Symbol>, Weight)>>>,
        index: TransitionTableIndex,
    ) {
        // Endles loop protection
        if cursor.lock().unwrap().overflowed() {
            return;
        }

        if self.max_analyses > 0 && total_analyses.lock().unwrap().len() >= self.max_analyses {
            return;
        }

        if index >= TRANSITION_TARGET_TABLE_START {
            let index = index - TRANSITION_TARGET_TABLE_START;
            self.try_transitions(
                input_string,
                cursor.clone(),
                Arc::clone(&total_analyses),
                index+1,
                EPSILON,
            );

            if input_string.is_empty() {
                if let Some(tr) = self.transitions.get(index as usize).filter(|tr| tr.is_final()) {
                    let mut cursor = cursor.lock().unwrap();
                    cursor.add_weight(tr.weight());
                    total_analyses.lock().unwrap().push(cursor.dump());
                    cursor.take_weight(tr.weight());
                }
                return;
            }

            self.try_transitions(
                input_string,
                cursor,
                total_analyses,
                index+1,
                input_string[0],
            )
        } else {
            if let Some(tr) = self.index.get(index as usize + 1).filter(|tr| tr.symbol() == EPSILON) {
                self.try_transitions(
                    input_string,
                    Arc::clone(&cursor),
                    Arc::clone(&total_analyses),
                    tr.target() - TRANSITION_TARGET_TABLE_START,
                    EPSILON,
                );
            }

            if input_string.is_empty() {
                if let Some(tr) = self.index.get(index as usize).filter(|tr| tr.is_final()) {
                    let mut cursor = cursor.lock().unwrap();
                    cursor.add_weight(tr.weight());
                    total_analyses.lock().unwrap().push(cursor.dump());
                    cursor.take_weight(tr.weight());
                }
                return;
            }

            if let Some(tr) = self.index.get(index as usize + input_string[0] as usize + 1)
                .filter(|tr| tr.symbol() == input_string[0])
            {
                self.try_transitions(
                    input_string,
                    cursor,
                    total_analyses,
                    tr.target() - TRANSITION_TARGET_TABLE_START,
                    input_string[0],
                )
            }
        }
    }
}

impl Transducer for WeightedTransducer {
    fn lookup_encoded(&self, input: &[Symbol]) -> Result<Vec<(Vec<Symbol>, Weight)>, Error> {
        let cursor = Arc::new(Mutex::new(Cursor::new()));
        let total_analyses = Arc::new(Mutex::new(Vec::new()));
        self.analyze(input, cursor, total_analyses.clone(), 0);
        Arc::into_inner(total_analyses).ok_or(Error::SyncError)
            .and_then(|mutex| mutex.into_inner().map_err(|_| Error::SyncError))
    }

    fn key_table(&self) -> &KeyTable {
        self.alphabet.key_table()
    }

    fn input_letters(&self) -> &Trie<u16> {
        &self.input_letters
    }

    fn set_max_analyses(&mut self, count: usize) {
        self.max_analyses = count;
    }
}
