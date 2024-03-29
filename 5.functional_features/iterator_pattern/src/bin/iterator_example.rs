fn main() {}

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => format!("{}{}", first.to_uppercase(), c.as_str()),
    }
}

pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|w| capitalize_first(w)).collect()
}

pub fn capitalize_words_string(words: &[&str]) -> String {
    words
        .iter()
        .map(|w| capitalize_first(w))
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("input"), "Input");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "word"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "Word"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "word"];
        assert_eq!(capitalize_words_string(&words), "Hello Word");
    }
}
