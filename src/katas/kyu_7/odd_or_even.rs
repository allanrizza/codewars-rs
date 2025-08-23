fn odd_or_even(numbers: Vec<i32>) -> String {
    let mut sum: i32 = 0;
    for i in 0..numbers.len() {
       sum+=numbers[i]; 
    }
    if sum%2==0 {
        return "even".to_string();
    }
    return "odd".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_tests() {
        assert_eq!(odd_or_even(vec![]), "even");
    }

    #[test]
    fn test_single_even() {
        assert_eq!(odd_or_even(vec![0]), "even");
    }

    #[test]
    fn test_single_odd() {
        assert_eq!(odd_or_even(vec![1]), "odd");
    }

    #[test]
    fn test_even() {
        assert_eq!(odd_or_even(vec![0, 1, 5]), "even");
    }

    #[test]
    fn test_odd() {
        assert_eq!(odd_or_even(vec![0, 1, 4]), "odd");
    }

    #[test]
    fn test_negative_even() {
        assert_eq!(odd_or_even(vec![0, -1, -5]), "even");
    }


    #[test]
    fn test_negative_odd() {
        assert_eq!(odd_or_even(vec![0, -1, 2]), "odd");
    }
}
