
fn main() {
    let a = make_upper_case("abcd");
    println!("{a}")
}

fn make_upper_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    s.as_bytes().iter()
        .for_each(|c| {
            match c {
                97..=122 => result.push((c & !32u8) as char),
                _ => result.push(*c as char)
            }
        });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_upper_case() {
        assert_eq!(make_upper_case("hello"), "HELLO");
    }
}
