

fn calculate(data : &Vec<String>, dX : usize, dY : usize) -> u32 {
    let mut number_of_trees = 0;
    let mut x_coord = 0;
    let mut y_coord = 0;
    while y_coord < data.len() {
        number_of_trees += if data[y_coord].chars().cycle().nth(x_coord).unwrap_or('.') == '#' { 1 } else { 0 };
        x_coord = x_coord + dX % data.len();
        y_coord += dY;
    }
    
    number_of_trees as u32
}

pub fn run_part1(data : &Vec<String>) -> u32 {
    calculate(data, 3, 1)
}

pub fn run_part2(data : &Vec<String>) -> u32 {
    // there is a way to do this in one loop, but I just want to reuse my existing code rather than write two bespoke functions.
    calculate(data, 1, 1) * calculate(data, 3, 1) * calculate(data, 5, 1) * calculate(data, 7, 1) * calculate(data, 1, 2)
}