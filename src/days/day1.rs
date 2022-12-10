use std::io::{BufRead, BufReader, Read};
use std::error::Error;

pub fn input_1_to_vec<R: Read>(io: R, check: i32) -> Result<i64,Box<dyn Error>> {
    let bufread = BufReader::new(io);

    let mut v = Vec::new();

    let mut temp_sum:i64 = 0;

    for line in bufread.lines() {

        match line {
            Ok(l) => {
                if l.is_empty(){
                    v.push(temp_sum);
                    temp_sum = 0;
                } else {
                    match l.trim().parse::<i64>() {
                        Ok(i) => temp_sum += i,
                        Err(e) => return Err(Box::new(e))
                    }
                }
            }
            Err(_) => {}
        }
    }

    let mut n_vec:Vec<i64> = Vec::new();

    for _ in 0..check {
        let max = v.iter().max().unwrap();
        let index = v.iter().position(|n| n == max).unwrap();

        n_vec.push(v.remove(index))
    }

    Ok(n_vec.iter().sum())
}