
fn words(phrase: &str) -> std::str::SplitWhitespace {
    return phrase.split_whitespace()
}

pub fn initials(phrase: &str) -> String {
    words(phrase).map(|word|
                      word.chars().next().unwrap()
    ).collect::<String>().to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words() {
        let input = "this is the way\n the world ends";
        let expect = vec!["this", "is", "the", "way", "the", "world", "ends"];
        assert_eq!(words(input).collect::<Vec<&str>>(), expect)
    }

    #[test]
    fn test_initials() {
        let input = "not with a bang   but a whimper";
        assert_eq!(initials(input), "NWABBAW");
    }

    #[test]
    fn empty_words() {
        let input = "";
        let expect: Vec<&str> = Vec::new();
        assert_eq!(words(input).collect::<Vec<&str>>(), expect);
    }

    #[test]
    fn empty_initials() {
        let input = "";
        assert_eq!(initials(input), "");
    }
}
