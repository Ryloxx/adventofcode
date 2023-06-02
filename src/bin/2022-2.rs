use adventofcode::{display_result, read_input};
fn part_one(input: Vec<String>) -> i32 {
    input
        .iter()
        .map(|x| {
            let mut split = x.split(" ");
            let combination = (
                split.next().unwrap().as_bytes()[0],
                split.next().unwrap().as_bytes()[0],
            );
            let x = combination.0 - b'A';
            let y = combination.1 - b'X';
            (1 + y + 3 * ((y + 4 - x) % 3)) as i32
        })
        .sum()
}
fn part_two(input: Vec<String>) -> i32 {
    input
        .iter()
        .map(|x| {
            let mut split = x.split(" ");
            let combination = (
                split.next().unwrap().as_bytes()[0],
                split.next().unwrap().as_bytes()[0],
            );
            let x = combination.0 - b'A';
            let y = combination.1 - b'X';
            1 + match y {
                0 => (x + 2) % 3,
                1 => x + 3,
                2 => (((x + 3) - 2) % 3) + 6,
                _ => unreachable!(),
            } as i32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use adventofcode::InputString;

    const INPUT: &str = r#"A Y
B X
C Z"#;
    #[test]
    fn part_one() {
        assert_eq!(
            super::part_one(InputString(INPUT.to_string()).parse_as_list_of_string(),),
            15
        )
    }
    #[test]
    fn part_two() {
        assert_eq!(
            super::part_two(InputString(INPUT.to_string()).parse_as_list_of_string(),),
            12
        )
    }
}

fn main() {
    display_result(
        Some(part_one(read_input(2022, 2, 1).parse_as_list_of_string())),
        Some(part_two(read_input(2022, 2, 1).parse_as_list_of_string())),
    )
}
