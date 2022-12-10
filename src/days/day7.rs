// Went through several iterations, including trying to develop a tree structure, which worked but
// was unwieldy. Solution ultimately: record the current working directory as a string "a.bc.def"
// and keep a hashmap of directories with name as a key and

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

pub(crate) fn process<R>(io: R) -> (u32,u32) where R: Read {

    let mut curr_dir: String = String::new();
    let mut dir_hash: HashMap<String, u32> = HashMap::new();

    let bufread = BufReader::new(io);

    let lines = bufread.lines().map(|l| l.unwrap());

    for line in lines.filter(|l| l != "$ ls" && &l[0..3] != "dir") {
        let line_vec: Vec<&str> = line.split_whitespace().collect();

        // now there are only two line types left, $ cd and # filename
        if line_vec[0] == "$" {
            // is a cd
            match line_vec[2] {
                ".." => {
                    curr_dir.truncate(curr_dir.rfind(".").unwrap());
                },
                _ => {
                    curr_dir.push('.');
                    curr_dir.push_str(line_vec[2]);
                },
            }

        } else {
            // Is a file listing
            let filesize: u32 = line_vec[0].parse().unwrap();

            // Get the entry for the current working directory, add filesize to it
            // If we don't have an entry for this directory yet (first visit), init
            let dsize = dir_hash.entry(curr_dir.clone()).or_insert(0);
            *dsize += filesize;

            // directory sizes also reflect the size of their directory children
            for (i, _) in curr_dir.match_indices('.') {
                let mut parent_dir = curr_dir.clone();
                parent_dir.truncate(i);
                let dsize = dir_hash.entry(parent_dir).or_insert(0);
                *dsize += filesize;
            }
        }
    }
    let under_max: Vec<_> = dir_hash.values().filter(|&d| d < &100000).collect();

    let fs_size = dir_hash["./"];
    let unused = 70000000 - fs_size;
    let needed = 30000000 - unused;

    let over_needed: Vec<_> = dir_hash.values().filter(|&d| d > &needed).collect();

    return (under_max.into_iter().sum(), *over_needed.into_iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use crate::days::day7::process;

    #[test]
    fn test_process() {
        let res1 = process(File::open("./input/day7_test.txt").unwrap());
        assert_eq!(res1, (95437, 24933642));
    }
}
