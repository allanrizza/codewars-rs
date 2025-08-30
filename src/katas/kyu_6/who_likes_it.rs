fn likes(names: &[&str]) -> String {
    if names.is_empty() {
        return "no one likes this".to_string();
    }

    return match names.len() {
        1 => format!("{} likes this", names[0]),
        2 => format!("{} and {} like this", names[0], names[1]),
        3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
        _ => format!("{}, {} and {} others like this", names[0], names[1], names.len()-2),
    }
}
