use std::io;

fn main() {
    println!("Risp Version 0.1.0-SNAPSHOT");
    println!("Press Ctrl-C to exit");
    let mut reader = io::stdin();

    loop {
        print!("> ");
        let input = reader.read_line().ok().expect("Failed to read line");
        print!("{:s}", input);
    }
}
