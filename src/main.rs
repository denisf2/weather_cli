use std::env;

fn print_cli_help() {
    // write using help
    println!("Getting forcast\r\n");
    println!("Usage:\r\n\tpass as arguments city and country code");
    println!("\texample: London GB");
}

fn main() {
    // get cli args
    let args = env::args().collect::<Vec<String>>();
    dbg!(&args);

    if args.len() < 2 {
        print_cli_help();
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
