use std::env;

fn main() {
    // write using help
    println!("pass as arguments city and country code");
    println!("example: London UK");
    
    // get cli args
    let args = env::args().collect::<Vec<String>>();

    dbg!(args);
}
