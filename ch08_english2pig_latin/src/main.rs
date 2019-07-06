fn english2pig_latin(s: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_words_with_first_consonant() {
        assert_eq!(english2pig_latin("first"), "irst-fay");
    }

    #[test]
    fn converts_words_with_first_vowel() {
        assert_eq!(english2pig_latin("apple"), "apple-hay");
    }

    #[test]
    fn converts_words_with_non_alphabetical_characters() {
        assert_eq!(english2pig_latin("five-year-old"), "ive-year-old-fay");
    }

    #[test]
    fn converts_one_or_two_letter_words() {
        assert_eq!(english2pig_latin("I"), "I-hay");
        assert_eq!(english2pig_latin("me"), "e-may");
    }

    #[test]
    fn converts_non_wordy_string_to_the_same_string() {
        assert_eq!(english2pig_latin(""), "");
        assert_eq!(english2pig_latin("- _"), "- _");
    }
}

fn main() {}
