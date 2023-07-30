use alloc::{
    boxed::Box,
    str,
};
use array_init::array_init;

const SIZE: usize = (u8::MAX as usize) + 1;

/// Trie data structure for storing values associated with prefix strings.
///
/// Nodes of the trie represent one byte of a UTF-8-encoded string.
///
/// Search is done on the longest prefix.  That is, given a string,
/// [`Trie::get()`] would return the value stored for the longest part that is a
/// prefix of the given string.
///
/// ```
/// use hfstol::trie::Trie;
/// 
/// let mut trie = Trie::new(None);
/// trie.insert("he", -2);
/// trie.insert("hello", 5);
/// assert_eq!(trie.get("hello world"), (Some(&5), " world"));
/// ```
pub struct Trie<V> {
    value: Option<V>,
    descendants: [Option<Box<Trie<V>>>; SIZE],
}

impl<V> Trie<V> {
    pub fn new(value: Option<V>) -> Trie<V> {
        Trie {
            value,
            descendants: array_init(|_| None),
        }
    }

    pub fn insert_by_bytes_key(&mut self, key: &[u8], value: V) {
        if key.is_empty() {
            self.value = Some(value);
        }
        else if let Some(ref mut sub) = self.descendants[key[0] as usize] {
            sub.insert_by_bytes_key(&key[1..], value);
        } else {
            let mut sub = Box::new(Trie::new(None));
            sub.insert_by_bytes_key(&key[1..], value);
            self.descendants[key[0] as usize] = Some(sub);
        }
    }

    /// Inserts a value associated with a string key.
    pub fn insert(&mut self, key: &str, value: V) {
        self.insert_by_bytes_key(key.as_bytes(), value)
    }

    /// Returns `true` if there exists a key starting with specified `char`.
    pub fn has_key_starting_with(&self, char: u8) -> bool {
        self.descendants[char as usize].is_some()
    }

    pub fn get_by_bytes_key<'a, 'b>(&'a self, key: &'b [u8]) -> (Option<&'a V>, &'b [u8]) {
        if key.is_empty() {
            (self.value.as_ref(), key)
        }
        else if let Some(ref sub) = self.descendants[key[0] as usize] {
            let (found, rest) = sub.get_by_bytes_key(&key[1..]);
            if found.is_some() {
                (found, rest)
            } else {
                (self.value.as_ref(), key)
            }
        } else {
            (self.value.as_ref(), key)
        }
    }

    /// Returns the value associated with the longest prefix of `key` and the
    /// remaining part of `key`.
    ///
    /// If nothing at all could be found, [`None`] is passed as the first
    /// element of the tuple.
    pub fn get<'a, 'b>(&'a self, key: &'b str) -> (Option<&'a V>, &'b str) {
        let (found, rest) = self.get_by_bytes_key(key.as_bytes());
        (found, unsafe { str::from_utf8_unchecked(rest) })
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;
    
    #[test]
    fn prefix_search() {
        let mut trie = Trie::new(None);
        trie.insert("or", 4);
        trie.insert("orange", 20);
        assert_eq!(trie.get("orange juice"), (Some(&20), " juice"));
    }

    #[test]
    fn fallback() {
        let trie = Trie::new(Some(10));
        assert_eq!(trie.get("string"), (Some(&10), "string"));
    }
}
