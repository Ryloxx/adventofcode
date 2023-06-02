use std::collections::BinaryHeap;

use adventofcode::{display_result, read_input};

fn part_one(input: Vec<Vec<String>>) -> i32 {
    return input
        .iter()
        .map(|x| {
            x.iter()
                .map(|x| {
                    x.as_str()
                        .parse::<i32>()
                        .expect(format!("Expect: {x} to be parsable as number").as_str())
                })
                .sum()
        })
        .max()
        .unwrap_or(0);
}
fn part_two(input: Vec<Vec<String>>) -> i32 {
    let heap: BinaryHeap<i32> = input
        .iter()
        .map(|x| {
            x.iter()
                .map(|x| {
                    x.as_str()
                        .parse::<i32>()
                        .expect(format!("Expect: {x} to be parsable as number").as_str())
                })
                .sum()
        })
        .collect();
    return heap.iter().take(3).sum();
}

#[cfg(test)]
mod tests {
    use adventofcode::InputString;
    const INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
    #[test]
    fn part_one() {
        assert_eq!(
            super::part_one(InputString(INPUT.to_string()).parse_as_2d_list_of_string(),),
            24000
        )
    }
    #[test]
    fn part_two() {
        assert_eq!(
            super::part_two(InputString(INPUT.to_string()).parse_as_2d_list_of_string(),),
            45000
        )
    }
}

fn main() {
    display_result(
        Some(part_one(
            read_input(2022, 1, 1).parse_as_2d_list_of_string(),
        )),
        Some(part_two(
            read_input(2022, 1, 1).parse_as_2d_list_of_string(),
        )),
    )
}
