fn count_bits(n: i64) -> u32 {
    let mut i: i64;
    let mut num: i64 = n;
    let mut count: u32 = 0;

    while num > 0  {
        i = num % 2;
        if i > 0 {
            count += 1;
        }
        num = num / 2;
    }

    return count;
}

// n.count_ones
