#![allow(dead_code)]
mod brainfuck;

use std::io;

static HELLO_WORLD: &'static str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
static CAT: &'static str = ",[.,]";

fn main() {
    brainfuck::brainfuck(String::from(CAT), Box::new(io::stdin()), Box::new(io::stdout()));
}
