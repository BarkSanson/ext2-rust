use std::{env, process};

use ext2_rust::blocks::{ FileSystem, };

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("mkfs: 2 arguments must be provided. Call to program must have next structure: ");
        eprintln!("mkfs <path> <number_of_blocks>");
        process::exit(1);
    }

    let path = &args[1];

    let _system = FileSystem::new(path);
}