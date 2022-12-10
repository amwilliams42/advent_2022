use std::error::Error;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
struct Command {
    number: usize,
    stack_from: usize,
    stack_to: usize,
}

pub(crate) struct CraneRun{
    stacks: Vec<Vec<char>>,
    commands: Vec<Command>,
}

impl Command{
    pub fn from_line(line: String) -> Self {
        //move 2 from 2 to 8
        let command: Vec<usize> = line.split_whitespace().flat_map(|x| x.parse()).collect();

        Command{
            number: command[0],
            stack_from: command[1] -1,
            stack_to: command[2] -1,
        }
    }
}

impl CraneRun{
    pub fn new<R: Read>(io: R) -> Result<Self, Box<dyn Error>> {
        let bufread = BufReader::new(io);

        let mut pre_stack: Vec<String>= vec![]; //collection for the stack definition lines
        let mut processing_stack: bool = true;

        let mut commands: Vec<Command> = vec![];
        for line in bufread.lines(){
            match line {
                Ok(l) => {
                    match processing_stack{
                        true => {
                            if l.is_empty() {
                                processing_stack = false;
                            } else {
                                pre_stack.push(l);
                            }
                        }
                        false => {
                            commands.push(Command::from_line(l));
                        }
                    }
                }
                Err(_) => {}
            }
        }
        //takes the last line of the pre-stack, splits it up, parses it, then returns the max
        let stack_count = pre_stack.pop()
            .unwrap()
            .split_whitespace()
            .flat_map(|x| x.parse::<i32>())
            .max()
            .unwrap();

        let mut stacks: Vec<Vec<char>> = Vec::with_capacity(stack_count as usize);
        (0..stack_count).for_each(|_| {
            let stack = Vec::new();
            stacks.push(stack)
        });

        let rows = pre_stack.iter().rev();

        for row in rows{
            let letters = row.chars()
                .skip(1)
                .step_by(4);
            for (i, letter) in letters.enumerate() {
                if !letter.is_whitespace() {stacks[i].push(letter)}
            }
        }
        let c = CraneRun{ stacks, commands };
        Ok(c)
    }
    pub fn run(&mut self) {
        for command in &self.commands{
            for _ in 0..command.number {
                let c = self.stacks[command.stack_from].pop().unwrap();
                self.stacks[command.stack_to as usize].push(c);
            }
        }
    }
    pub fn run2(&mut self) {
        for command in &self.commands{
            let split_index = self.stacks[command.stack_from].len() - command.number;
            let mut moving = self.stacks[command.stack_from].split_off(split_index);

            self.stacks[command.stack_to].append(&mut moving);
        }
    }

    pub fn tops(&self) -> String {
        let mut tops: Vec<char> = vec![];
        for stack in &self.stacks{
            tops.push(stack.last().unwrap().clone())
        }
        return String::from_iter(tops.iter())
    }
}