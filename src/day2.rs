
//This is very brittle. Probably should add much better parsing capable of handling some errors
fn parse_data(s : &str) -> ((u64, u64), char, String) {
    
    //7-9 l: vslmtglbc

    let tokens = s.split_whitespace().collect::<Vec<_>>();
    let range = tokens[0].split('-').collect::<Vec<_>>();

    ((range[0].parse().unwrap(),range[1].parse().unwrap()), tokens[1].chars().nth(0).unwrap(), tokens[2].to_string())
}

pub fn run_part1(data : &Vec<String>) -> u64 {

    data.iter().filter(|x| {
        let ((min, max), letter, pw) = parse_data(x);
            let count = pw.chars().filter(|&y| y == letter).count();

            // I just want to be able to chain comparisons. Why is this not supported in most languages?
            min <= count as u64 && count as u64<= max

    }).count() as u64

}

pub fn run_part2(data : &Vec<String>) -> u64 {

    data.iter().filter(|x| {
        let ((fst, snd), letter, pw) = parse_data(x);
            (pw.chars().nth(fst as usize - 1 ).unwrap() == letter) ^ (pw.chars().nth(snd as usize - 1).unwrap() == letter)
    }).count() as u64

}