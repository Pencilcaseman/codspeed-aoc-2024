macro_rules! day_wrapper {
    ($target_day:tt, $output_day:tt) => {
        pub mod $output_day {
            use aoc::year2024::$target_day as solution;
            use std::fmt::Display;

            pub fn part1(input: &str) -> impl Display {
                solution::part1(&solution::parse(input))
            }

            pub fn part2(input: &str) -> impl Display {
                solution::part2(&solution::parse(input))
            }
        }
    };
}

day_wrapper!(day01, day1);
day_wrapper!(day02, day2);
day_wrapper!(day03, day3);
day_wrapper!(day04, day4);
day_wrapper!(day05, day5);
day_wrapper!(day06, day6);
day_wrapper!(day07, day7);
