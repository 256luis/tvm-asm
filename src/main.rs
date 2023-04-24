use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

enum Opcode {
    Pushl,
    Pusha,
    Pop,
    Addi,
    Addf,
    Subi,
    Subf,
    Muli,
    Mulf,
    Divi,
    Divf,
    Jmp,
    Jz,
    Jnz,
    Jg,
    Jge,
    Jl,
    Jle,
    And,
    Or,
    Not,
    Xor,
    Call,
    Ret,
    Out,
    Halt,
}

struct Instruction {
    opcode: Opcode,
    operand: Option<u32>,
}

fn line_to_instruction(line: &String) -> Result<Instruction, String> {
    let tokens: Vec<String> = line.split(' ').map(|x| x.to_string()).collect();
    let opcode_string = tokens.get(0).unwrap().as_str();
    
    let opcode = match opcode_string {
        "PUSHL" => Ok(Opcode::Pushl),
        "PUSHA" => Ok(Opcode::Pusha),
        "POP" => Ok(Opcode::Pop),
        "ADDI" => Ok(Opcode::Addi),
        "ADDF" => Ok(Opcode::Addf),
        "SUBI" => Ok(Opcode::Subi),
        "SUBF" => Ok(Opcode::Subf),
        "MULI" => Ok(Opcode::Muli),
        "MULF" => Ok(Opcode::Mulf),
        "DIVI" => Ok(Opcode::Divi),
        "DIVF" => Ok(Opcode::Divf),
        "JMP" => Ok(Opcode::Jmp),
        "JZ" => Ok(Opcode::Jz),
        "JNZ" => Ok(Opcode::Jnz),
        "JG" => Ok(Opcode::Jg),
        "JGE" => Ok(Opcode::Jge),
        "JL" => Ok(Opcode::Jl),
        "JLE" => Ok(Opcode::Jle),
        "AND" => Ok(Opcode::And),
        "OR" => Ok(Opcode::Or),
        "NOT" => Ok(Opcode::Not),
        "XOR" => Ok(Opcode::Xor),
        "CALL" => Ok(Opcode::Call),
        "RET" => Ok(Opcode::Ret),
        "OUT" => Ok(Opcode::Out),
        "HALT" => Ok(Opcode::Halt),
        _ => Err(format!("Invalid opcode: {}", opcode_string)),
    }?;

    // extract operand (if applicable)
    let mut operand: Option<u32> = None;
    {
        match opcode {
            // must have operand
            Opcode::Pushl |
            Opcode::Pusha | 
            Opcode::Pop |
            Opcode::Jmp |
            Opcode::Jz  |
            Opcode::Jnz |
            Opcode::Jg  |
            Opcode::Jge |
            Opcode::Jl  |
            Opcode::Jle |
            Opcode::Call => {
                // check if number of operands is correct
                if tokens.len() != 2 {
                    Err(format!("Instruction {}, must have one operand.", opcode_string))?
                }

                operand = match tokens.get(1).unwrap().parse::<u32>() {
                    Ok(value) => Some(value),
                    Err(_) => todo!()
                }
            },

            // everything else must not have operand
            _ => {
                // check if number of operands is correct
                if tokens.len() != 1 {
                    Err(format!("Instruction {}, must not have any operands.", opcode_string))?
                }

                operand = None;
            },
        };
    }
    
    Ok(Instruction {
        opcode,
        operand,
    })
}

fn main() {
    let source_code;
    {
        let args: Vec<String> = env::args().collect();
        let file_path = match args.get(1) {
            Some(s) => s,
            None => {
                println!("No file specified.");
                return;
            }
        };

        source_code = match fs::read_to_string(file_path) {
            Ok(s) => s,
            Err(_) => {
                println!("Error reading file.");
                return;
            }
        };
    }

    // get each line of code
    let source_lines: Vec<String> = source_code
        .lines()
        .map(|x| x.to_uppercase())
        .collect();

    // output bytes
    let mut output_bytes: Vec<u8> = Vec::new();

    // main compilation loop
    for line in source_lines.iter() {
        // convert each line to Instructions
        let inst = match line_to_instruction(&line.trim().to_string()) {
            Ok(i) => i,
            Err(msg) => {
                println!("ERROR: {}", msg);
                return
            }
        };

        // convert the Instruction to its binary representation
        let operand_bytes = inst.operand.unwrap_or(0).to_le_bytes();
        output_bytes.extend([
            inst.opcode as u8,
            operand_bytes[0],
            operand_bytes[1],
            operand_bytes[2],
            operand_bytes[3],
        ]);        
    }

    // write bytes to file
    let mut output_file = File::create("out.bin").unwrap();
    output_file.write_all(&output_bytes[..]);
}
