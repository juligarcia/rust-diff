use std::env;
use std::process;

pub mod data_structures;
pub mod my_fs;

use my_fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Use: ./program file_name_1 file_name_2");
        process::exit(1);
    }

    let filename1 = &args[1];
    let filename2 = &args[2];

    let file = File::new(filename1);
    let file2 = File::new(filename2);

    println!("{}", file.diff(&file2));
}
