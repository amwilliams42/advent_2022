#![feature(iter_next_chunk)]
use std::io::Error;
use std::fs::File;

mod day1;
mod day2;
mod day3;

fn main() -> Result<(),Error>{

    match day1::input_1_to_vec(File::open("./input/day1_1.txt")?, 3) {
        Ok(i) => {println!("{:?}", i)}
        Err(_) => {}
    }

    match day2::Tournament::new(File::open("./input/day2.txt")?) {
        Ok(t) => {println!("{:?}", t.score(false))}
        Err(_) => {}
    }
    match day2::Tournament::new(File::open("./input/day2.txt")?) {
        Ok(t) => {println!("{:?}", t.score(true))}
        Err(_) => {}
    }
    match day3::pack_values(File::open("./input/day3.txt")?, false) {
        Ok(p) => {println!("{:?}", p)}
        Err(_) => {}
    }
    Ok(())
}
