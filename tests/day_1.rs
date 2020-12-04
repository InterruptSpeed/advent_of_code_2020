#[cfg(test)]
mod day_1 {
    use advent_of_code_2020 as advent;

    const FILENAME: &str = "./data/day_1_input.txt";

    #[test]
    fn part_1_works() {
        let result = advent::solve_day_1a(FILENAME);
        assert!(result > 0);
    }

    #[test]
    fn part_2_works() {
        let result = advent::solve_day_1b(FILENAME);
        assert!(result > 0); 
    }
}