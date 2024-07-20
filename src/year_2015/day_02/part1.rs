pub fn solution(input: &str) -> String {
    input
    .lines()
    .map(|line| -> i32 {
        let dimensions: Vec<i32> = line.split('x')
        .map(|x| {
            str::parse::<i32>(x).unwrap_or_default()
        })
        .collect();
        let [x, y, z]: [i32; 3] = dimensions.try_into().ok().unwrap_or_default();
        let sides: Vec<i32> = vec![x*y, x*z, y*z];
        let smallest_side: i32 = match sides.iter().min() {
            Some(value) => *value,
            None => 0
        };
        2 * sides.iter().sum::<i32>() + smallest_side
    })
    .sum::<i32>()
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("2x3x4"), "58");
        assert_eq!(solution("1x1x10"), "43");
    }
}
