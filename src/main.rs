mod day1; use day1::*;
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn day1() {
    let data : Vec<i32> = lines_from_file("./input/day1.txt").expect("Could not load data file")
    .iter_mut()
    .map(|x| x.parse().unwrap()).collect();

    println!("{}", match run_part1(&data, 2020) {Some(x) => x, _ => 0});
    println!("{}", match run_part2(&data, 2020) {Some(x) => x, _ => 0});
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            println!("Expected an arguement to indicate what day to run: E.G. 1");
        },
        // one argument passed
        2 => {
            match args[1].parse() {
                Ok(day) => match day {
                    1 => day1(),
                    _ => println!("{} is an invald day number", day)
                },
                _ => println!("Invalid argument. expected day number")
            }
        },
        _ => println!("Invalid number of arguments. Expected only a day number")
    }

}
