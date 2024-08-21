enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

pub fn process_input_part1(input: &str) -> String {
    let result =  "one";
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part_1_work() {
        let result = process_input_part1(INPUT);
        assert_eq!(result, "");
    }
}
