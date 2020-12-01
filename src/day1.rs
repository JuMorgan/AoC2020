use std::{
    collections::HashSet
};

//Two Sums
pub fn run_part1(data : &Vec<i32>, target : i32) -> Option<i32>{

    let mut seen = HashSet::new();

    for x in data.iter() {
        let val = target - x;

        if seen.contains(&val) {
            return Some(x * val);
        }

        seen.insert(x);
    }

    None
}

//Three Sums
pub fn run_part2(data : &Vec<i32>, target : i32) -> Option<i32>{

    let mut seen = HashSet::new();

    for (i,x) in data.iter().enumerate() {
        for j in i+1..data.len() {  
            let val = target - x - data[j];

            if seen.contains(&val) {
                return Some(x * data[j] * val);
            }
        }

        seen.insert(x);
    }

    None
}