use std::collections::HashMap;

fn duplicate_encode(word:&str) -> String {
    let mut char_count = HashMap::new();

    for c in word.chars().map(|c| c.to_ascii_lowercase()) {
        *char_count.entry(c).or_insert(0) += 1;
    }

    word.chars()
        .map(|c| {
            if char_count[&c.to_ascii_lowercase()] > 1 { ')' } else { '(' }
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_tests() {
      assert_eq!(duplicate_encode("din"),"(((");
      assert_eq!(duplicate_encode("recede"),"()()()");
      assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
      assert_eq!(duplicate_encode("(( @"),"))((");
    }
}
