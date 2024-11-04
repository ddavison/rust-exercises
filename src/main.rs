fn main() {
    let a = make_upper_case("abcd");
    println!("{a}");
    let b = true;

    make_upper_case("test");
}

fn make_upper_case(s: &str) -> String {
    s.chars()
        .map(|c| match c as u8 {
            (97..=122) => c as u8 - 32,
            _ => c as u8,
        } as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_upper_case() {
        assert_eq!(make_upper_case("hello"), "HELLO");
    }
}
