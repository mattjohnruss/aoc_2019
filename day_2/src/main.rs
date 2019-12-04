use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let program = load_program("input")?;

    println!("{}", part_1(&program));
    println!("{}", part_2(&program)?);

    Ok(())
}

fn part_1(program: &[u32]) -> u32 {
    let mut program = program.to_owned();

    program[1] = 12;
    program[2] = 2;

    run_program(&mut program);
    program[0]
}

fn part_2(program: &[u32]) -> Result<u32> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = program.to_owned();
            program[1] = noun;
            program[2] = verb;
            run_program(&mut program);

            if program[0] == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err("No noun and verb combination found!".into())
}

fn run_program(program: &mut[u32]) {
    let mut inst_ptr = 0;
    while inst_ptr < program.len() {
        match program[inst_ptr] {
            1 => {
                let arg_1 = program[program[inst_ptr + 1] as usize];
                let arg_2 = program[program[inst_ptr + 2] as usize];
                program[program[inst_ptr + 3] as usize] = arg_1 + arg_2;
                inst_ptr += 4;
            }
            2 => {
                let arg_1 = program[program[inst_ptr + 1] as usize];
                let arg_2 = program[program[inst_ptr + 2] as usize];
                program[program[inst_ptr + 3] as usize] = arg_1 * arg_2;
                inst_ptr += 4;
            }
            99 => break,
            opcode => {
                panic!("Unknown opcode: {}", opcode)
            }
        }
    }
}

fn load_program(filename: &str) -> Result<Vec<u32>> {
    let program: std::result::Result<Vec<_>, _> =
        fs::read_to_string(filename)?
        .trim()
        .split(",")
        .map(|x| x.parse::<u32>())
        .collect();
    program.map_err(|e| e.into())
}
