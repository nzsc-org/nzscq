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

pub fn has_duplicates<T: Ord>(v: &mut Vec<T>) -> bool {
    let len = v.len();
    v.sort_unstable();
    v.dedup();
    let unique_len = v.len();

    len != unique_len
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
        let mut fibonacci: Vec<u8> = vec![1, 1, 2, 3, 5];
        assert!(has_duplicates(&mut fibonacci));
    }

    #[test]
    fn abc_does_not_have_duplicates() {
        let mut abc: Vec<char> = vec!['a', 'b', 'c'];
        assert!(!has_duplicates(&mut abc));
    }
}
