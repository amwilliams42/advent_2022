use std::cmp::max;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};
use std::ops::Sub;

#[derive(Debug, Clone)]
struct  Command {
    direction: Direction,
    distance: i32,
}
#[derive(Debug, Clone)]
enum Direction {R,L,U,D}
type Result<C> = std::result::Result<C, CommandParseError>;

#[derive(Debug, Clone)]
struct CommandParseError;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Point(i32,i32);

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x_d = self.0 - rhs.0;
        let y_d = self.1 - rhs.1;
        Point(x_d,y_d)
    }
}

impl Point {
    fn c_dist(&self, other:Point) -> i32{
        let xd = self.0 - other.0;
        let yd = self.1 - other.1;
        max(xd.abs(), yd.abs())
    }
}

impl Command{
    pub fn from_line(line: String) -> Result<Self> {
        use Direction::{R,L,U,D};
        //move 2 from 2 to 8
        let command: Vec<_> = line.split_whitespace().collect();
        match (command[0], command[1].parse::<i32>().unwrap()) {
            ("R", i) => return Ok(Self{direction: R,distance:i}),
            ("L", i) => return Ok(Self{direction: L,distance:i}),
            ("U", i) => return Ok(Self{direction: U,distance:i}),
            ("D", i) => return Ok(Self{direction: D,distance:i}),

            _ => { Err(CommandParseError)}
        }
    }
}

pub(crate) fn simulate_bridge<R>(io: R) -> i32 where R: Read {

    let mut cmds: Vec<Command> = Vec::with_capacity(2000);
    let bufread = BufReader::new(io);
    let lines = bufread.lines().map(|l| l.unwrap());
    for line in lines {
        cmds.push(Command::from_line(line).unwrap());
    }

    let mut tail_positions: HashSet<Point> = HashSet::new();
    let mut head: Point = Point(0,0);
    let mut tail: Point = Point(0,0);

    tail_positions.insert(tail);

    for cmd in cmds {
        // we will do n moves
        for _ in 0..cmd.distance {
            // First move the head
            match cmd.direction {
                Direction::R => {head.0 += 1}
                Direction::L => {head.0 -= 1}
                Direction::U => {head.1 += 1}
                Direction::D => {head.1 -= 1}
            }
            //determine if the tail needs moved

            let diff = head - tail;
            let cdist = head.c_dist(tail);

            if cdist > 1 {
                // They are not touching, The form will be (1,2),(-1,2),(1,-2),(-1,-2)
                match diff {
                    // Diagonal
                    Point(1, 2) | Point(2,1) => {
                        tail.0 += 1;
                        tail.1 += 1;
                    },
                    Point(-1, 2) | Point(-2,1) => {
                        tail.0 -= 1;
                        tail.1 += 1;
                    },
                    Point(1, -2) | Point(2, -1) => {
                        tail.0 += 1;
                        tail.1 -= 1;
                    },
                    Point(-1, -2) | Point(-2, -1)=> {
                        tail.0 -= 1;
                        tail.1 -= 1;
                    },
                    //they are in line, so one direction will be 0. Form (0,2),(0,-2),(2,0),(-2,0)
                    Point(0, 2) => {
                        tail.1 += 1;
                    },
                    Point(0, -2) => {
                        tail.1 -= 1;
                    },
                    Point(2, 0) => {
                        tail.0 += 1;
                    },
                    Point(-2, 0) => {
                        tail.0 -= 1;
                    },


                    p => panic!("unexpected point difference {:?}", p)
                }
                tail_positions.insert(tail);
            }
        }
    }
    return tail_positions.len() as i32
}


#[cfg(test)]
mod tests {
    use crate::days::day9::{Point, simulate_bridge};

    #[test]
    fn test_process() {
        let input = "R 4
                            U 4
                            L 3
                            D 1
                            R 4
                            D 1
                            L 5
                            R 2";
        let res1 = simulate_bridge(input.as_bytes());
        assert_eq!(res1, 13);
    }
    #[test]
    fn test_distance() {
        let p1 = Point(31,44);
        let p2 = Point(4,16);
        let origin = Point(0,0);
        let p3 = Point(1,1);

        assert_eq!(p1 - p2, Point(27,28));
        assert_eq!(origin - p3, Point(-1,-1));
        assert_eq!(p3 - origin, Point(1,1));

        assert_eq!(p1.c_dist(p2), 28);
        assert_eq!(origin.c_dist(p3), 1);
        assert_eq!(p3.c_dist(origin),1);
    }
}
