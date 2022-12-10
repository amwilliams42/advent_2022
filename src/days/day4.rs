use std::collections::HashSet;
use std::error::Error;
use std::io::{BufRead, BufReader, Read};

pub fn overlap<R: Read>(io: R) -> Result<(i32,i32),Box<dyn Error>> {
    let bufread = BufReader::new(io);

    //Each line is 2 ranges a-b and c-d. Create two hashsets per line and check if super/subset

    let line_iter = bufread.lines();

    let mut contains_count = 0;
    let mut nondisjoint_count = 0;

    for line in line_iter {
        match line {
            Ok(s) => {
                //a-b,c-d
                let v: Vec<&str> = s.as_str().split(|c| c == '-' || c == ',').collect();
                let mut ab: Vec<i32> = v.iter().map(|n| n.parse::<i32>().unwrap()).collect();
                let cd: Vec<i32> = ab.split_off(2);

                let range1: Vec<i32> = (ab[0]..ab[1]+1).collect();
                let range2: Vec<i32> = (cd[0]..cd[1]+1).collect();

                let range1_hash: HashSet<i32> = HashSet::from_iter(range1);
                let range2_hash: HashSet<i32> = HashSet::from_iter(range2);

                if range1_hash.is_subset(&range2_hash) || range1_hash.is_superset(&range2_hash){
                    contains_count += 1;
                }
                if !range1_hash.is_disjoint(&range2_hash) {
                    nondisjoint_count += 1
                }
            }
            Err(_) => {}
        }
    }
    return Ok((contains_count,nondisjoint_count))
}