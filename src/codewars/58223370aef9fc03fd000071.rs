fn main() {
    dashatize(274);
}

/// Given an integer, return a string with dash '-' marks before and after each odd digit,
/// but do not begin or end the string with a dash mark.
/// ```
/// 274 -> '2-7-4'
/// 6815 -> '68-1-5'
/// ```
fn dashatize(n: i64) -> String {
    let n = n.abs();
    let digits: Vec<char> = n.to_string().chars().collect();
    let is_odd = |n: &char| n.to_digit(10).unwrap() % 2 == 1;
    let mut result = String::new();

    for (i, c) in digits.iter().enumerate() {
        if is_odd(c) {
            if i > 0 { result.push('-') };
            result.push(*c);
            if !is_odd(digits.get(i + 1).unwrap_or(&'1')) { result.push('-') }
        } else { result.push(*c); }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(dashatize(274), "2-7-4");
        assert_eq!(dashatize(5311), "5-3-1-1");
        assert_eq!(dashatize(86320), "86-3-20");
        assert_eq!(dashatize(974302), "9-7-4-3-02");
    }

    #[test]
    fn weird() {
        assert_eq!(dashatize(0), "0");
        assert_eq!(dashatize(-1), "1");
        assert_eq!(dashatize(-28369), "28-3-6-9");
        assert_eq!(dashatize(545235814), "5-4-5-2-3-5-8-1-4");
    }
}
