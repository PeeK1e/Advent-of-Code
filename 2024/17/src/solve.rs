use std::time::Instant;

#[allow(unused_variables,unused_mut)]
pub fn solve_t1(input: &str) -> Result<String, String> {
    let mut count = 0;
    let (mut cpu, ops) = CPU::new(input);

    let start = Instant::now();
    while (cpu.ip as usize) < ops.len() {
        let operand = ops[(cpu.ip+1) as usize];
        let num =        || match operand {
            0..=3 => operand,
            4 => cpu.rega,
            5 => cpu.regb,
            6 => cpu.regc,
            _ => unreachable!()
        };
        match ops[cpu.ip as usize] {
            0 => cpu.rega = cpu.rega >> num() as u64,
            1 => cpu.regb = cpu.regb ^ num() as u64,
            2 => cpu.regb = (num() as u64) % 8,
            3 => {
                if cpu.rega != 0 {
                    cpu.ip = num() as usize;
                    continue;
                }
            },
            4 => cpu.regb = cpu.regb ^ cpu.regc,
            5 => cpu.buff.push(num() % 8),
            6 => cpu.regb = cpu.rega >> num() as u64,
            7 => cpu.regc = cpu.rega >> num() as u64,
            _ => unreachable!(),
        };
        cpu.ip += 2;
    }

    let end = Instant::now() - start;
    println!("Took: {}micro sec.", end.as_nanos());

    let out = cpu.buff.iter().map(|ele| ele.to_string()).collect::<Vec<String>>().join(",");

    Ok(out)
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t2(input: &str) -> Result<String, String> {
    Ok("".to_owned())
}

#[derive(Clone)]
struct CPU {
    ip: usize,
    rega: u64,
    regb: u64,
    regc: u64,
    buff: Vec<u64>,
}

impl CPU {
    fn new(input: &str) -> (Self, Vec<u64>) {
        let lines = input.split("\n\n").collect::<Vec<&str>>();
        let register = lines[0].split("\n").map(|line|{
            let reg = line.split(" ").collect::<Vec<&str>>();
            reg[2].parse::<u64>().unwrap()
        })
        .collect::<Vec<u64>>();
        let operations = lines[1].split(" ")
            .collect::<Vec<&str>>()[1]
            .split(",")
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let cpu = Self{
            ip: 0,
            rega: register[0],
            regb: register[1],
            regc: register[2],
            buff: vec![],
        };
        (cpu, operations)
    }
}
