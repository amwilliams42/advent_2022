#![feature(iter_next_chunk)]
use std::io::Error;
use std::fs::File;
use std::path::Path;
use clap::{Parser};
mod days;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Day to run
    #[arg(value_parser = clap::value_parser!(u16).range(1..=25))]
    day: u16,
    #[arg(long)]
    p2: bool,
}


fn main() -> Result<(),Error> {

    let cli = Cli::parse();

    let day_num:u16 = cli.day;
    let path = Path::new(".").join("input").join(format!("day{}.txt", day_num));
    let file = File::open(path).unwrap();
    match cli.day {
        1 => {
            println!("{:?}", days::day1::input_1_to_vec(file, 3).unwrap())
        },
        2 => {
            let t = days::day2::Tournament::new(file).unwrap();
            println!("{:?}", t.score(cli.p2))
        }
        3 => {
            println!("{:?}", days::day3::pack_values(file, cli.p2).unwrap())
        },
        4 => {
            let (o,d) = days::day4::overlap(file).unwrap();
            println!("Overlap:{:?}, Non-Disjoint {:?}", o, d)
        },
        5 => {
            let mut cr = days::day5::CraneRun::new(file).unwrap();
            if !cli.p2 {
                cr.run()
            } else {
                cr.run2()
            }
            println!("{:?}", cr.tops())
        },
        6 => {
            let (p,m) = days::day6::start_of_packet(file);
            println!("Packet start: {:?}, message start: {:?}",p,m)
        },
        7 => {
            let (u, o) = days::day7::process(file);
            println!("Sum of under 100k: {:?}, Smallest Dir to Delete: {:?}", u, o)
        },
        8 => {

        }
        9 => {
            println!("Number of visited positions: {:?}", days::day9::simulate_bridge(file))
        }
        _ => {
            println!("Day not yet implemented")
        },
    }
    Ok(())
}
