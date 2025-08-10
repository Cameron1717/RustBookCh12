use mini_grep;
use std::env;
use std::fs;
use std::process;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = mini_grep::Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments:{err}");
        process::exit(1);
    });
}
