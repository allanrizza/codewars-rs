fn alphabet_position(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphabetic() {
                // c as u8: converte o caractere para seu valor ASCII
                // b'a': valor numérico de 'a'
                // c as u8 - b'a': cálculo da posição
                Some((c as u8 - b'a' + 1).to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_examples() {
        assert_eq!(
            alphabet_position("The sunset sets at twelve o' clock."),
            "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string(),
        );
        assert_eq!(
            alphabet_position("The narwhal bacons at midnight."),
            "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string(),
        );
    }

    #[test]
    fn test_letters() {
        assert_eq!(
            alphabet_position("abcdefghijklmnopqrstuvwxyz"),
            "1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26",
        );
        assert_eq!(
            alphabet_position("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            "1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26",
        );
    }

    #[test]
    fn test_symbols() {
        assert_eq!(
            alphabet_position(r#"-=!@#$%^&*()_+[];,./\{}:|<>? "#),
            "",
        );
    }
}
