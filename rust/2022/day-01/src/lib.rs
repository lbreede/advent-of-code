pub fn process_part1(input: &str) -> String {
    let result = input
        .split("\n\n")
        .map(|entry| {
            entry
                .lines()
                .map(|number| number.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut result = input
        .split("\n\n")
        .map(|entry| {
            entry
                .lines()
                .map(|number| number.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    result.sort_by(|a, b| b.cmp(a));
    let sum: u32 = result.iter().take(3).sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_works() {
        let result = process_part1(EXAMPLE);
        assert_eq!(result, "24000");
    }
    #[test]
    fn part2_works() {
        let result = process_part2(EXAMPLE);
        assert_eq!(result, "45000");
    }
}
