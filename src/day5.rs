
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

pub fn run(mut data: Vec<String>) -> (usize, usize) {
    fn go((x,y) : (usize,usize), (i, b) : (usize, char)) -> (usize, usize) {
        match b {
            'F' => {(x,y)},
            'B' => {(x + usize::pow(2, (6 - i) as u32), y)},
            'L' => {(x,y)},
            'R' => {(x, y + usize::pow(2, (9 - i) as u32))},
            _ => {(x,y)}
        }
    }
    let seats : Vec<_>= data.iter_mut()
        .map(|x| {let (a,b) = x.chars()
                  .enumerate()
                  .fold((0,0), go); 
                  a* 8 + b}).collect();

    let mut result : (usize, usize) = (0,0);
    match seats.iter().minmax() {
        MinMax(&min, &max) => {
            result.0 = max;
            result.1 = (max * (max + 1) / 2) - seats.iter().sum::<usize>() - ((min - 1) * min / 2);

        },
        _ =>{panic!("Unexpected: no min or max value present in seat list");}
    }

    result
}
