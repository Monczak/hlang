use std::{io::{self, Write, Read}, env, fs::File};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut memory = vec![0u8; 30000];
    let mut pointer: usize = 0;

    let instructions: Vec<&str> = contents.split_whitespace().collect();
    match interpret(instructions, &mut memory, &mut pointer) {
        Err(e) => panic!("{}", e),
        _ => {}
    }

    Ok(())
}

fn interpret(instructions: Vec<&str>, memory: &mut Vec<u8>, pointer: &mut usize) -> Result<(), &'static str> {
    let mut loop_stack = Vec::new();
    let mut instruction_pointer: usize = 0;
    
    while instruction_pointer < instructions.len() {
        let instr = instructions[instruction_pointer];
        let instr_size = instr.chars().count();
        for c in instr.chars() {
            if c != 'h' {
                return Err("invalid instruction")
            } 
        }

        let instr_result = match instr_size {
            1 => increment_at_pointer(memory, *pointer),
            2 => decrement_at_pointer(memory, *pointer),
            3 => increment_pointer(memory, pointer),
            4 => decrement_pointer(memory, pointer),
            5 => start_loop(memory, pointer, &mut instruction_pointer, &instructions, &mut loop_stack),
            6 => end_loop(memory, pointer, &mut instruction_pointer, &mut loop_stack),
            7 => print(memory, pointer),
            8 => input(memory, pointer),
            _ => return Err("invalid instruction")
        };
        match instr_result {
            Err(e) => panic!("{}", e),
            _ => {}
        }

        instruction_pointer += 1;
    };

    Ok(())
}

fn increment_at_pointer(memory: &mut Vec<u8>, pointer: usize) -> Result<(), &'static str> {
    memory[pointer] = memory[pointer].wrapping_add(1);
    Ok(())
}

fn decrement_at_pointer(memory: &mut Vec<u8>, pointer: usize) -> Result<(), &'static str> {
    memory[pointer] = memory[pointer].wrapping_sub(1);
    Ok(())
}

fn increment_pointer(memory: &Vec<u8>, pointer: &mut usize) -> Result<(), &'static str> {
    *pointer += 1;
    if *pointer >= memory.len() {
        *pointer = 0;
    }
    Ok(())
}

fn decrement_pointer(memory: &Vec<u8>, pointer: &mut usize) -> Result<(), &'static str> {
    if *pointer == 0 {
        *pointer = memory.len() - 1;
    }
    else {
        *pointer -= 1;
    }
    Ok(())
}

fn start_loop(memory: &mut Vec<u8>, pointer: &mut usize, instruction_pointer: &mut usize, instructions: &Vec<&str>, loop_stack: &mut Vec<usize>) -> Result<(), &'static str> {
    if memory[*pointer] == 0 {
        let mut loop_count = 1;
        while loop_count > 0 {
            *instruction_pointer += 1;
            if instruction_pointer >= &mut instructions.len() {
                return Err("loop not closed");
            }

            let next_instr = instructions[*instruction_pointer];
            if next_instr.chars().count() == 5 {
                loop_count += 1;
            }
            else if next_instr.chars().count() == 6 {
                loop_count -= 1;
            }
        }
    }
    else {
        loop_stack.push(*instruction_pointer);
    }

    Ok(())
}

fn end_loop(memory: &mut Vec<u8>, pointer: &mut usize, instruction_pointer: &mut usize, loop_stack: &mut Vec<usize>) -> Result<(), &'static str> {
    if memory[*pointer] != 0 {
        if let Some(&last_loop_start) = loop_stack.last() {
            *instruction_pointer = last_loop_start;
        }
        else {
            return Err("loop stack empty");
        }
    }
    else {
        loop_stack.pop();
    }

    Ok(())
}

fn print(memory: &mut Vec<u8>, pointer: &mut usize) -> Result<(), &'static str> {
    print!("{}", memory[*pointer] as char);
    io::stdout().flush().unwrap();

    Ok(())
}

fn input(memory: &mut Vec<u8>, pointer: &mut usize) -> Result<(), &'static str> {
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read input");

    if let Some(data) = input_text.trim().chars().next() {
        memory[*pointer] = data as u8;
    }
    else {
        return Err("failed to parse input");
    }

    Ok(())
}