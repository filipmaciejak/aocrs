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
    .scan((0, 0), |state, x: i32| -> Option<(i32, i32)> {
        *state = (state.0 + 1, state.1 + x);
        Some(*state)
    })
    .filter(|(_, height)| {
        *height < 0
    })
    .map(|(index, _)| -> i32 {
        index
    })
    .take(1)
    .collect::<Vec<i32>>()[0]
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(")"), "1");
        assert_eq!(solution("()())"), "5");
    }
}
