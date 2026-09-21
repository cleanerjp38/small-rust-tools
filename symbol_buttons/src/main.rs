use std::io;

fn main() {
    let symbols = ["√", "π", "θ", "∑"];

    for (i, symbol) in symbols.iter().enumerate() {
        println!("{}: {}", i + 1, symbol);
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let choice: usize = input.trim().parse().unwrap();

    println!("選択: {}", symbols[choice - 1]);
}