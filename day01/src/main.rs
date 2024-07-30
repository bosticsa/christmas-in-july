fn main() {
    println!("Day 1 data retrieval initiated....");
    let input_data = include_str!("./input1.txt");
    let output = acquire_calibration_value(input_data);
    println!("The calibration value is {}", output.to_string())
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
}
