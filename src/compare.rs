pub fn compare_string(x: &str, y: &str) -> bool {
    for (a, b) in x.chars().zip(y.chars()) {
        match a.cmp(&b) {
            std::cmp::Ordering::Less => return false,
            std::cmp::Ordering::Greater => return true,
            std::cmp::Ordering::Equal => (),
        }
    }
    x.len() > y.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_string() {
        assert_eq!(compare_string("abc", "abcc"), false);
        assert_eq!(compare_string("abc", "abc"), false);
        assert_eq!(compare_string("abc", "abd"), false);
        assert_eq!(compare_string("abc", "ab"), true);
        assert_eq!(compare_string("abc", "abb"), true);
    }
}
