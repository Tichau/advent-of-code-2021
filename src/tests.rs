#[cfg(test)]
mod tests {
    use std::io;
    use std::path::Path;
    use std::fs::File;
    use crate::{DaySolver, Solver};
    use crate::solvers::*;

    fn open_test_file(day: i32) -> io::BufReader<std::fs::File> {
        let path = Path::new("data").join(format!("day{:02}.txt", day));
        io::BufReader::new(File::open(&path).expect("Failed to read file"))
    }

    #[test]
    fn day01() {
        let mut solver = DaySolver::from(day01::parser, day01::part1, day01::part2);
        solver.parse(open_test_file(1));
        assert_eq!(solver.solve_part1(), 1162, "Part1");
        assert_eq!(solver.solve_part2(), 1190, "Part2");
    }

    #[test]
    fn day02() {
        let mut solver = DaySolver::from(day02::parser, day02::part1, day02::part2);
        solver.parse(open_test_file(2));
        assert_eq!(solver.solve_part1(), 1648020, "Part1");
        assert_eq!(solver.solve_part2(), 1759818555, "Part2");
    }
    
    #[test]
    fn day03() {
        let mut solver = DaySolver::from(day03::parser, day03::part1, day03::part2);
        solver.parse(open_test_file(3));
        assert_eq!(solver.solve_part1(), 3901196, "Part1");
        assert_eq!(solver.solve_part2(), 4412188, "Part2");
    }

    #[test]
    fn day04() {
        let mut solver = DaySolver::from(day04::parser, day04::part1, day04::part2);
        solver.parse(open_test_file(4));
        assert_eq!(solver.solve_part1(), 8442, "Part1");
        assert_eq!(solver.solve_part2(), 4590, "Part2");
    }

    #[test]
    fn day05() {
        let mut solver = DaySolver::from(day05::parser, day05::part1, day05::part2);
        solver.parse(open_test_file(5));
        assert_eq!(solver.solve_part1(), 6267, "Part1");
        assert_eq!(solver.solve_part2(), 20196, "Part2");
    }

    #[test]
    fn day06() {
        let mut solver = DaySolver::from(day06::parser, day06::part1, day06::part2);
        solver.parse(open_test_file(6));
        assert_eq!(solver.solve_part1(), 380758, "Part1");
        assert_eq!(solver.solve_part2(), 1710623015163, "Part2");
    }

    #[test]
    fn day07() {
        let mut solver = DaySolver::from(day07::parser, day07::part1, day07::part2);
        solver.parse(open_test_file(7));
        assert_eq!(solver.solve_part1(), 349769, "Part1");
        assert_eq!(solver.solve_part2(), 99540554, "Part2");
    }

    #[test]
    fn day08() {
        let mut solver = DaySolver::from(day08::parser, day08::part1, day08::part2);
        solver.parse(open_test_file(8));
        assert_eq!(solver.solve_part1(), 488, "Part1");
        assert_eq!(solver.solve_part2(), 1040429, "Part2");
    }

    #[test]
    fn day09() {
        let mut solver = DaySolver::from(day09::parser, day09::part1, day09::part2);
        solver.parse(open_test_file(9));
        assert_eq!(solver.solve_part1(), 560, "Part1");
        assert_eq!(solver.solve_part2(), 959136, "Part2");
    }

    #[test]
    fn day10() {
        let mut solver = DaySolver::from(day10::parser, day10::part1, day10::part2);
        solver.parse(open_test_file(10));
        assert_eq!(solver.solve_part1(), 299793, "Part1");
        assert_eq!(solver.solve_part2(), 3654963618, "Part2");
    }

    #[test]
    fn day11() {
        let mut solver = DaySolver::from(day11::parser, day11::part1, day11::part2);
        solver.parse(open_test_file(11));
        assert_eq!(solver.solve_part1(), 1702, "Part1");
        assert_eq!(solver.solve_part2(), 251, "Part2");
    }

    #[test]
    fn day12() {
        let mut solver = DaySolver::from(day12::parser, day12::part1, day12::part2);
        solver.parse(open_test_file(12));
        assert_eq!(solver.solve_part1(), 4720, "Part1");
        assert_eq!(solver.solve_part2(), 147848, "Part2");
    }

    #[test]
    fn day13() {
        let mut solver = DaySolver::from(day13::parser, day13::part1, day13::part2);
        solver.parse(open_test_file(13));
        assert_eq!(solver.solve_part1(), 775, "Part1");
    }

    #[test]
    fn day14() {
        let mut solver = DaySolver::from(day14::parser, day14::part1, day14::part2);
        solver.parse(open_test_file(14));
        assert_eq!(solver.solve_part1(), 2027, "Part1");
        assert_eq!(solver.solve_part2(), 2265039461737, "Part2");
    }

    #[test]
    fn day15() {
        let mut solver = DaySolver::from(day15::parser, day15::part1, day15::part2);
        solver.parse(open_test_file(15));
        assert_eq!(solver.solve_part1(), 415, "Part1");
        assert_eq!(solver.solve_part2(), 2864, "Part2");
    }

    #[test]
    fn day16() {
        let mut solver = DaySolver::from(day16::parser, day16::part1, day16::part2);
        solver.parse(open_test_file(16));
        assert_eq!(solver.solve_part1(), 1012, "Part1");
        assert_eq!(solver.solve_part2(), 2223947372407, "Part2");
    }

    #[test]
    fn day17() {
        let mut solver = DaySolver::from(day17::parser, day17::part1, day17::part2);
        solver.parse(open_test_file(17));
        assert_eq!(solver.solve_part1(), 3655, "Part1");
        assert_eq!(solver.solve_part2(), 1447, "Part2");
    }

    #[test]
    fn day18() {
        let mut solver = DaySolver::from(day18::parser, day18::part1, day18::part2);
        solver.parse(open_test_file(18));
        assert_eq!(solver.solve_part1(), 4137, "Part1");
        assert_eq!(solver.solve_part2(), 4573, "Part2");
    }
}
