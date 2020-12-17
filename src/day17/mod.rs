
pub mod coordinates;
pub mod space;
use space::*;

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = simulate_times(input, 3, 6);
    return result as i64;
}

fn simulate_times(input: &str, dimensions: usize, times: usize) -> i64 {
    let mut space = Space::from_input(input, dimensions);

    for _ in 0..times {
        space.simlate_cycle();
    }

    let result = space.count_actives();
    return result as i64;
}

fn get_challenge_input() -> &'static str {
    ".###.#.#
####.#.#
#.....#.
####....
#...##.#
########
..#####.
######.#
"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        ".#.
..#
###"
    }

    #[test]
    fn example_get_part1_result() {
        let input = get_example_input();
        let result = simulate_times(input, 3, 6);

        assert_eq!(112, result);
    }

    #[test]
    fn input_get_part1_result() {
        let result = get_part1_result();

        assert_eq!(388, result);
    }
}
