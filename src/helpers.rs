use std::collections::HashSet;
use std::hash::Hash;

pub fn lowercase_no_whitespace(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut s = String::new();

    for &byte in bytes.iter() {
        let c = byte as char;

        if !c.is_whitespace() {
            s.push(c);
        }
    }

    s.to_lowercase()
}

pub trait HasDuplicates {
    fn has_duplicates(&self) -> bool;
}

impl<T: Eq + Hash> HasDuplicates for Vec<T> {
    fn has_duplicates(&self) -> bool {
        let mut set = HashSet::new();
        let is_unique = self.iter().all(|item| set.insert(item));
        !is_unique
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercase_no_whitespace_works() {
        assert_eq!(lowercase_no_whitespace("Hello world"), "helloworld");
    }

    #[test]
    fn fibonacci_sequence_has_duplicates() {
        let fibonacci: Vec<u8> = vec![1, 1, 2, 3, 5];
        assert!(fibonacci.has_duplicates());
    }

    #[test]
    fn abc_does_not_have_duplicates() {
        let abc: Vec<char> = vec!['a', 'b', 'c'];
        assert!(!abc.has_duplicates());
    }
}
