use std::error::Error;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use crate::day2::MyMove::{X, Y, Z};
use crate::day2::OppMove::{A, B, C};

pub struct Tournament {
    rounds: Vec<Round>
}

struct Round {
    op: OppMove,
    my: MyMove,
}

enum OppMove {
    A,
    B,
    C,
}
enum MyMove{
    X,
    Y,
    Z,
}

impl FromStr for OppMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(OppMove::A),
            "B" => Ok(OppMove::B),
            "C" => Ok(OppMove::C),
            _ => Err(())
        }
    }
}

impl FromStr for MyMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(MyMove::X),
            "Y" => Ok(MyMove::Y),
            "Z" => Ok(MyMove::Z),
            _ => Err(())
        }
    }
}

impl Round {
    fn new(op: OppMove,my: MyMove) -> Self {
        Round{
            op,
            my
        }
    }
    fn score(&self) -> i32 {
        match (&self.op, &self.my) {
            (A,X) => {4},// Rock v Rock: 1 + 3 = 4
            (A,Y) => {8},// Rock v Paper: 2 + 6 = 8
            (A,Z) => {3},// Rock v Scissors: 3 + 0 = 3
            (B,X) => {1},// Paper v Rock: 1 + 0 = 1
            (B,Y) => {5},// Paper v Paper: 2 + 3 = 5,
            (B,Z) => {9},// Paper v Scissors: 3 + 6 = 9,
            (C,X) => {7},// Scissors v Rock: 1 + 6 = 7
            (C,Y) => {2},// Scissors v  Paper: 2 + 0 = 3,
            (C,Z) => {6},// Scissors v Scissors: 3 + 3 = 6,
        }
    }

    fn score2(&self) -> i32 {
        match &self.my {
            //Lose
            X => {
                match &self.op {
                    A => { 3 + 0} //Play Scissors and Lose
                    B => { 1 + 0} //Play Rock and Lose
                    C => { 2 + 0} //Play Paper and Lose
                }
            }
            //Draw
            Y => {
                match &self.op {
                    A => { 1 + 3} //Play Rock and Draw
                    B => { 2 + 3} //Play Paper and Draw
                    C => { 3 + 3} //Play Scissors and Draw
                }
            }
            //Win
            Z => {
                match &self.op {
                    A => { 2 + 6} //Play Paper and Win
                    B => { 3 + 6} //Play Scissors and Win
                    C => { 1 + 6} //Play Rock and Win
                }
            }
        }
    }
}

impl Tournament {
    pub fn new<R: Read>(io: R) -> Result<Self,Box<dyn Error>> {
        let mut rounds: Vec<Round> = Vec::new();

        let bufread = BufReader::new(io);

        for line in bufread.lines() {
            match line {
                Ok(l) => {
                    let mut iter = l.split_whitespace();
                    let op_s = iter.next().unwrap();
                    let my_s = iter.next().unwrap();
                    let op = OppMove::from_str(op_s).unwrap();
                    let my = MyMove::from_str(my_s).unwrap();
                    rounds.push(Round::new(op,my))
                }
                Err(_) => {}
            }
        }

        Ok(Tournament{
            rounds
        })
    }

    pub fn score(&self, p2: bool) -> i32 {
        let mut i= 0;
        for round in &self.rounds{
            match p2 {
                true => { i += round.score2()}
                false => {i += round.score()}
            }
        }
        return i
    }


}