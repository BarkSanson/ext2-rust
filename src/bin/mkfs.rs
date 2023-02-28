use std::{env, process, io};

use ext2_rust::blocks::{ FileSystem, };

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("mkfs: 2 arguments must be provided. Call to program must have next structure: ");
        eprintln!("mkfs <path> <number_of_blocks>");
        process::exit(1);
    }

    let path = &args[1];
    let number_of_blocks: usize = match args[2].parse::<usize>() {
        Ok(num) => num,
        Err(e) => {
            eprintln!("mkfs: number of blocks must be a positive integer");
            process::exit(1)
        }
    };

    let mut system = FileSystem::new(path)?;

    let buff = [0x00; 1024];

    for i in 0..number_of_blocks {
        system.write_block(i as u64, &buff)?;
    }

    Ok(())
}