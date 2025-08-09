use std::env;
fn main() {
    println!("Hello, world!");
    let args: vec<String> = env::args().collect();
    bdg!(args);
}
