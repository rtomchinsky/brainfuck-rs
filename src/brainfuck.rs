use std::io::prelude::*;

type CellSize = u8;
const TAPE_SIZE: usize = 0x7FFF;

const BRAINFUCK_TOKEN_PLUS: char       = '+';
const BRAINFUCK_TOKEN_MINUS: char      = '-';
const BRAINFUCK_TOKEN_PREV: char       = '<';
const BRAINFUCK_TOKEN_NEXT: char       = '>';
const BRAINFUCK_TOKEN_PRINT: char      = '.';
const BRAINFUCK_TOKEN_READ: char       = ',';
const BRAINFUCK_TOKEN_LOOP_START: char = '[';
const BRAINFUCK_TOKEN_LOOP_END: char   = ']';

struct BrainfuckContext {
    tape: [CellSize; TAPE_SIZE],
    tape_size: usize,
    tape_index: usize,
    output: Box<Write>,
    input: Box<Read>
}

impl BrainfuckContext {
    fn new(input: Box<Read>, output: Box<Write>) -> BrainfuckContext {
        BrainfuckContext {
            tape: [0; TAPE_SIZE],
            tape_size: TAPE_SIZE,
            tape_index: 0,
            output: output,
            input: input
        }
    }

    fn shl(&mut self) {
        if self.tape_index == 0 {
            self.tape_index = self.tape_size - 1;
        } else {
            self.tape_index = self.tape_index - 1;
        }
    }

    fn shr(&mut self) {
        if self.tape_index == self.tape_size - 1 {
            self.tape_index = 0;
        } else {
            self.tape_index = self.tape_index + 1;
        }
    }

    fn incr(&mut self) {
        self.tape[self.tape_index] = self.tape[self.tape_index] + 1;
    }

    fn decr(&mut self) {
        self.tape[self.tape_index] = self.tape[self.tape_index] - 1;
    }

    fn curr(&self) -> u8 {
        return self.tape[self.tape_index];
    }

    fn read(&mut self) {
        let mut buffer = [0; 1];
        self.input.read(&mut buffer).unwrap();
        self.tape[self.tape_index] = buffer[0];
    }

    fn print(&mut self) {
        let buffer = &[self.curr()];
        self.output.write(buffer).unwrap();
    }
}

pub fn brainfuck(source: String, input: Box<Read>, output: Box<Write>) {
    let mut program = Vec::new();
    for ch in source.chars() {
        program.push(ch);
    }

    let mut pointer: usize = 0;
    let mut context = BrainfuckContext::new(input, output);
    let mut loop_stack: Vec<usize> = Vec::new();
    let mut skip = 0;

    while pointer < program.len() {
        let instruction = program[pointer];

        if skip != 0 {
            match instruction {
                BRAINFUCK_TOKEN_LOOP_START => { skip = skip + 1; },
                BRAINFUCK_TOKEN_LOOP_END => { skip = skip - 1; },
                _ => { /* Ignore */ }
            }
        } else {
            match instruction {
                BRAINFUCK_TOKEN_LOOP_START => {
                    if context.curr() != 0 {
                        loop_stack.push(pointer);
                    } else {
                        skip = 1;
                    }
                },
                BRAINFUCK_TOKEN_LOOP_END => {
                    if let Some(start) = loop_stack.pop() {
                        if context.curr() != 0 {
                            pointer = start - 1;
                        }
                    } else {
                        panic!("Unmatched ] in position {}", pointer);
                    }
                },
                BRAINFUCK_TOKEN_PREV => context.shl(),
                BRAINFUCK_TOKEN_NEXT => context.shr(),
                BRAINFUCK_TOKEN_MINUS => context.decr(),
                BRAINFUCK_TOKEN_PLUS => context.incr(),
                BRAINFUCK_TOKEN_PRINT => context.print(),
                BRAINFUCK_TOKEN_READ => context.read(),
                _ => { /* Ignore */ }
            }
        }

        pointer = pointer + 1;
    }
}