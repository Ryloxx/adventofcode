use std::{fmt::Debug, fs};

pub struct InputString(pub String);

pub fn read_input(year: u32, day: u32, part: u32) -> InputString {
    let file_name = format!("src/inputs/{year}-{day}-{part}.txt");
    let file_content = fs::read_to_string(file_name).unwrap();
    return InputString(file_content);
}

pub fn display_result<T>(part_one: Option<T>, part_two: Option<T>)
where
    T: Debug,
{
    if let Some(part_one) = part_one {
        println!("Part One: {:?}", part_one)
    }
    if let Some(part_two) = part_two {
        println!("Part Two: {:?}", part_two)
    }
}

impl InputString {
    pub fn parse_as_2d_list_of_string(&self) -> Vec<Vec<String>> {
        return self
            .0
            .trim()
            .split("\n\n")
            .map(|x| {
                x.trim()
                    .split("\n")
                    .map(String::from)
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();
    }
    pub fn parse_as_list_of_string(&self) -> Vec<String> {
        return self
            .0
            .trim()
            .split("\n")
            .map(String::from)
            .collect::<Vec<String>>();
    }
}
