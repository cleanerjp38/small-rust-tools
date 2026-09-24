use std::io;

fn main() {
    let symbols = ["√", "π", "θ", "∑", "≤"];
    let mut formula = String::new();

    println!("0: 完成");

    for (i, symbol) in symbols.iter().enumerate() {
        println!("{}: {}", i + 1, symbol);
    }

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let choice: usize = input.trim().parse().unwrap();

        if choice == 0 {
            break;
        }

        formula.push_str(symbols[choice - 1]);

        println!("選択: {}", formula);
    }
}