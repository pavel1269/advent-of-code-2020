
pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = get_not_matching(&input, 25);

    return result;
}

pub fn get_part2_result() -> i64 {
    let input = get_challenge_input();
    let result = get_not_matching_source(&input, 21806024);

    return result;
}

fn get_not_matching_source(numbers: &Vec<i64>, number: i64) -> i64 {
    let position = numbers.iter().position(|&num| num == number).unwrap();
    let mut sums: Vec<(i64, i64, i64)> = vec!();
    for index in 0..position - 1 {
        let current_number = numbers[index];
        // println!("index: [{}] = {}", index, current_number);
        if sums.len() > 0 {
            for sum_index in 0..sums.len() {
                let mut sum = &mut sums[sum_index];
                sum.2 += current_number;

                if sum.2 == number {
                    // println!("Found! {} + {}", sum.0, sum.1);
                    return sum.0 + sum.1;
                }

                if current_number > sum.0 {
                    sum.0 = current_number;
                }
                if current_number < sum.1 {
                    sum.1 = current_number;
                }
            }
            sums.retain(|sum| sum.2 < number);
        }
        sums.push((current_number, current_number, current_number));
        // println!("sums: {:?}", sums);
    }

    return -1;
}

fn get_not_matching(numbers: &Vec<i64>, preamble_size: usize) -> i64 {
    for index in preamble_size..numbers.len() {
        if !is_valid(numbers, preamble_size, index) {
            return numbers[index];
        }
    }

    panic!("Solution not found.");
}

fn is_valid(numbers: &Vec<i64>, preamble_size: usize, index: usize) -> bool {
    let current_number = numbers[index];

    // 1 2 3 4 5 6
    // 1-4 <-> x 6
    // 1 <-> 2-5 6
    for first_number_index in index - preamble_size..index - 1 {
        let first_number = numbers[first_number_index];
        for second_number_index in first_number_index..index {
            // println!("{} {} {}", index, first_number_index, second_number_index);
            let second_number = numbers[second_number_index];
            let sum = first_number + second_number;

            if current_number == sum {
                return true;
            }
        }
    }

    return false;
}

fn get_challenge_input() -> Vec<i64> {
    include_str!("./inputs/day09.txt").lines().collect::<Vec<&'static str>>().iter().map(|line| line.parse::<i64>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> Vec<i64> {
        vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576,
        ]
    }

    fn get_example2_input() -> Vec<i64> {
        vec![
            7759,
            9386,
            6681,
            9138,
            7978,
            9602,
            9840,
            14426,
            9845,
            10037,
            19175,
            1,
        ]
    }

    #[test]
    fn example_not_matching() {
        let input = get_example_input();
        let result = get_not_matching(&input, 5);

        assert_eq!(127, result);
    }

    #[test]
    fn example2_not_matching() {
        let input = get_example2_input();
        let result = get_not_matching(&input, 10);

        assert_eq!(1, result);
    }

    #[test]
    fn input_not_matching() {
        let input = get_challenge_input();
        let result = get_not_matching(&input, 25);

        assert_eq!(21806024, result);
    }

    #[test]
    fn example_not_matching_source() {
        let input = get_example_input();
        let result = get_not_matching_source(&input, 127);

        assert_eq!(62, result);
    }

    #[test]
    fn input_not_matching_source() {
        let input = get_challenge_input();
        let result = get_not_matching_source(&input, 21806024);

        assert_eq!(2986195, result);
    }
}
