use std::collections::HashSet;
use std::error::Error;
use std::io::{BufRead, BufReader, Read};


pub fn pack_values<R: Read>(io: R, is_p1:bool) -> Result<i32,Box<dyn Error>> {
    let bufread = BufReader::new(io);
    let mut total_value = 0;
    let mut line_iter = bufread.lines();
    match is_p1 {
        true => {
            for line in line_iter {
                match line {
                    Ok(l) => {
                        let (pack1, pack2) = l.split_at(l.len()/2);
                        let p1_set: HashSet<char> = pack1.chars().collect();
                        let p2_set: HashSet<char> = pack2.chars().collect();

                        let intersect: HashSet<_> = p1_set.intersection(&p2_set).collect();
                        let char = intersect.iter().next().unwrap();
                        let mut value = i32::from_str_radix(char.to_string().as_str(),36).unwrap();

                        match char.is_lowercase() {
                            true => {value -= 9;}
                            false => {value += 17}
                        }
                        total_value += value;
                    }
                    Err(_) => {}
                }
            }
        }
        false => {
            while let Ok(chunk) = line_iter.next_chunk::<3>() {
                let mut packs: Vec<HashSet<char>> = Vec::new();
                for c in chunk {
                    match c {
                        Ok(l) => {packs.push(l.chars().collect())}
                        Err(_) => {}
                    }
                }
                let intersection = packs
                    .iter()
                    .skip(1)
                    .fold(packs[0].clone(), |a, b| {
                        a.intersection(b).cloned().collect()
                    });
                let char = intersection.iter().next().unwrap();
                let mut value = i32::from_str_radix(char.to_string().as_str(),36).unwrap();

                match char.is_lowercase() {
                    true => {value -= 9;}
                    false => {value += 17}
                }
                total_value += value;
            }
        }
    }

    Ok(total_value)
}
