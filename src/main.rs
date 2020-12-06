mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

extern crate itertools;
extern crate nom;

use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn string_from_file(filename : impl AsRef<Path>) -> io::Result<String> {
 
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).expect("should have read a thing");

            Ok(content)
        },
        
        Err(e) => {
            Err(e)
        },
    }
}

fn day1() {
    let data : Vec<i32> = lines_from_file("./input/day1.txt").expect("Could not load data file")
    .iter_mut()
    .map(|x| x.parse().unwrap()).collect();

    println!("{}", match day1::run_part1(&data, 2020) {Some(x) => x, _ => 0});
    println!("{}", match day1::run_part2(&data, 2020) {Some(x) => x, _ => 0});
}

fn day2() {
    let data : Vec<String> = lines_from_file("./input/day2.txt").expect("Could not load data file");

    println!("{}", day2::run_part1(&data));
    println!("{}", day2::run_part2(&data));
}

fn day3() {
    let data : Vec<String> = lines_from_file("./input/day3.txt").expect("Could not load data file");

    println!("{}", day3::run_part1(&data));
    println!("{}", day3::run_part2(&data));
}

fn day4() {
    let data = string_from_file("./input/day4.txt").expect("Could not load data file");

    let split_data : Vec<&str> = data.split("\n\n").collect();

    println!("{}", day4::run(&split_data));
}


fn day5() {
    let data : Vec<String> = lines_from_file("./input/day5.txt").expect("Could not load data file");

    println!("{:#?}", day5::run(data.clone()));
}

fn day6() {
    let data = string_from_file("./input/day6.txt").expect("Could not load data file");

    let split_data : Vec<&str> = data.split("\n\n").collect();

    println!("{}", day6::run(split_data));
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
                    2 => day2(),
                    3 => day3(),
                    4 => day4(),
                    5 => day5(),
                    6 => day6(),
                    _ => println!("{} is an invald day number", day)
                },
                _ => println!("Invalid argument. expected day number")
            }
        },
        _ => println!("Invalid number of arguments. Expected only a day number")
    }

}
