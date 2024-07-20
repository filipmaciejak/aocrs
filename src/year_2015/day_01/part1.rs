pub fn solution(input: &str) -> String {
    input
    .chars()
    .map(|c: char| -> i32 {
        match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        }
    })
    .sum::<i32>()
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("(())"), "0");
        assert_eq!(solution("()()"), "0");
        assert_eq!(solution("((("), "3");
        assert_eq!(solution("(()(()("), "3");
        assert_eq!(solution("))((((("), "3");
        assert_eq!(solution("())"), "-1");
        assert_eq!(solution("))("), "-1");
        assert_eq!(solution(")))"), "-3");
        assert_eq!(solution(")())())"), "-3");
    }
}
