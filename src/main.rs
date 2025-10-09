use std::fs;
use std::io::{self, Read};
use std::time::Instant;

fn run_bf(code: &str) {
    let mut tape = vec![0u8; 30000];
    let mut ptr: usize = 0;
    let mut pc: usize = 0; //program counter
    let code_bytes = code.as_bytes();
    while pc < code_bytes.len() {
        match code_bytes[pc] as char {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => tape[ptr] = tape[ptr].wrapping_add(1),
            '-' => tape[ptr] = tape[ptr].wrapping_sub(1),
            '.' => print!("{}", tape[ptr] as char),
            ',' => {
                let mut input = [0u8];
                io::stdin().read_exact(&mut input).expect("neshto si tam");
                tape[ptr] = input[0];
            }
            '[' => {
                if tape[ptr] == 0 {
                    let mut loop_depth = 1;
                    while loop_depth > 0 {
                        pc += 1;
                        match code_bytes[pc] as char {
                            '[' => loop_depth += 1,
                            ']' => loop_depth -= 1,
                            _ => {}
                        }
                    }
                }
            }

            ']' => {
                if tape[ptr] != 0 {
                    let mut loop_depth = 1;
                    while loop_depth > 0 {
                        pc -= 1;
                        match code_bytes[pc] as char {
                            '[' => loop_depth -= 1,
                            ']' => loop_depth += 1,
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }

        pc += 1;
    }
}

fn main() {
    let code = fs::read_to_string("./benchmark.bf").expect("murzi me da praq error handling");
    let optimization_time = Instant::now();
    run_bf(&code);
    let duration = optimization_time.elapsed();
    println!("{:?}", duration)
}
