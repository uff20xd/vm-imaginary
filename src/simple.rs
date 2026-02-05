use std::collections::*;
use std::default;
#[derive(Default, Debug)]
pub struct StackMachine {
    stack: VecDeque<usize>,
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Add, Sub, 
    Push(usize),
    Eq,
    If,
    Jump,
    Stop,
}

impl StackMachine {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            ..default::Default::default()
        }
    }
    pub fn exec(&mut self) -> () {
        while self.instruction_pointer < self.instructions.len() {
            match self.instructions[self.instruction_pointer]{
                Instruction::Add => {
                    let a = self.stack.pop_back().unwrap();
                    let b = self.stack.pop_back().unwrap();
                    self.stack.push_back(a + b);
                },
                Instruction::Sub => {
                    let a = self.stack.pop_back().unwrap();
                    let b = self.stack.pop_back().unwrap();
                    self.stack.push_back(a - b);
                },
                Instruction::Push(val) => {
                    self.stack.push_back(val);
                },
                Instruction::Eq => {
                    let a = self.stack.pop_back().unwrap();
                    let b = self.stack.pop_back().unwrap();
                    self.stack.push_back((a == b) as usize);
                },
                Instruction::If => {
                    let a = self.stack.pop_back().unwrap() != 0;
                    if !a { self.instruction_pointer += 1;}
                },
                Instruction::Jump => {
                    let a = self.stack.pop_back().unwrap();
                    self.instruction_pointer = a;
                },
                Instruction::Stop => {
                    return;
                }
            }
            self.instruction_pointer += 1;
        }
    }
}
