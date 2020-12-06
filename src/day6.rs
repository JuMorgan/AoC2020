
pub fn run(data: Vec<&str>) -> usize {

    let mut total : usize = 0;
    for x in data {

        let mut val : usize = 0;
        let mut group : usize = usize::MAX; 
        for c in x.chars() {
            if c != '\n' {
                val |= usize::pow(2, (c as u8 - 97) as u32)
            }
            else {
                group &= val;
                val = 0;
            }
            
        }
        if val != 0 {group &= val;}
        total += group.count_ones() as usize;
    }
    
    total
}