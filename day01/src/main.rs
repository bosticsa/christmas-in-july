use itertools::Itertools;

fn main() {
    println!("Day 1 data retrieval initiated....");
    let input_data = include_str!("./input1.txt");
    let output1 = acquire_calibration_value(input_data);
    println!("The calibration value for Part1 is {}", output1.to_string());


    println!("Wait...we need to adjust the trebuchet...");
    let output2 = acquire_calibration_value2(&input_data);
    println!("The new calibration value for Part2 is {}", output2.to_string());
}

fn acquire_calibration_value(input_data: &str) -> u32 {
    let answer = input_data
        .lines()
        .map(|line| {
            let mut nums_only = line.chars().filter_map(|char| char.to_digit(10));
            let int1 = nums_only.next().expect("Need an integer pal");

            match nums_only.last() {
                Some(int2) => format!("{int1}{int2}"),
                None => format!("{int1}{int1}"), // edge case like treb7uchet
            }
            .parse::<u32>()
            .expect("Need a valid number pal")
        })
        .sum::<u32>();

    return answer;
}

const NUMS: &[&str] = &[
    "0", "zero", "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7",
    "seven", "8", "eight", "9", "nine",
];


fn acquire_calibration_value2(input_data: &str) -> u32 {
    let answer: usize = input_data
        .lines()
        .map(|line| {
            let (a, b) = NUMS
                .iter()
                .enumerate()
                .flat_map(|(i, &n)| line.match_indices(n).map(move |(idx, _)| (idx, i / 2)))
                .minmax()
                .into_option()
                .unwrap();
            a.1 * 10 + b.1
        })
        .sum();
    return answer as u32;
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let result = acquire_calibration_value(
            "1abc2
                   pqr3stu8vwx
                   a1b2c3d4e5f
                   treb7uchet",
        );
        assert_eq!(result, 142)
    }

    #[test]
    fn test_part2() {
        let result = acquire_calibration_value2(
            "two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen"
        );

        assert_eq!(result, 281);
    }
}
