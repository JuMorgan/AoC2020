

fn calculate(data : &Vec<String>, dx : usize, dy : usize) -> u32 {
    let mut number_of_trees = 0;
    let mut x_coord = 0;
    let mut y_coord = 0;
    while y_coord < data.len() {
        number_of_trees += if data[y_coord].chars().cycle().nth(x_coord).unwrap_or('.') == '#' { 1 } else { 0 };
        x_coord = x_coord + dx % data.len();
        y_coord += dy;
    }
    
    number_of_trees as u32
}

pub fn run_part1(data : &Vec<String>) -> u32 {
    calculate(data, 3, 1)
}

pub fn run_part2(data : &Vec<String>) -> u32 {
    [(1,1), (3,1), (5,1), (7,1), (1,2)].iter()
                                       .map(|(x, y)| calculate(data, *x, *y))
                                       .product()
}