fn digital_root(n: i64) -> i64 {
    if n < 10 {
        return n;
    }
    
    let mut curr = n;
    let mut sum = 0;
    
    while curr > 0 {
        sum+=curr%10;
        curr = curr/10;
    }
    
    return digital_root(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
    }
}
