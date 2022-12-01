fn sort_top_cal_elves(input: &str) -> Vec<u32> {
    let result: Vec<u32> = vec![0];
    let mut output: Vec<u32> = input.lines().fold(result, |mut cal_count, line| {
        if line.is_empty() {
            cal_count.push(0);
        } else {
            let length = cal_count.len();
            cal_count[length - 1] += line.parse::<u32>().unwrap();
        }
        cal_count
    });
    output.sort();
    output
}

pub fn part_one(input: &str) -> Option<u32> {
    let sorted_cal_elves = sort_top_cal_elves(input);
    Some(sorted_cal_elves[sorted_cal_elves.len() -1])

}

pub fn part_two(input: &str) -> Option<u32> {
    let sorted_cal_elves = sort_top_cal_elves(input);
    let top_3 = sorted_cal_elves.iter().rev().take(3).fold(0, |total_cal, cal| {
        cal + total_cal
    });
    Some(top_3)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
