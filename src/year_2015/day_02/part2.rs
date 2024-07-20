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
        let perimeters: Vec<i32> = vec![2*(x+y), 2*(x+z), 2*(y+z)];
        let smallest_perimeter: i32 = match perimeters.iter().min() {
            Some(value) => *value,
            None => 0
        };
        x * y * z + smallest_perimeter
    })
    .sum::<i32>()
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("2x3x4"), "34");
        assert_eq!(solution("1x1x10"), "14");
    }
}
