use std::io;

fn main() {
    let symbols = ["√", "π", "θ", "∑", "≤", "≥", "≠"];
    let mut formula = String::new();

    println!("0: 完成");
    println!("8: ←");
    println!("9: C（全部消す）");

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

        if choice == 8 {
            formula.pop();//pop()は末尾の1文字を取り除く
            println!("選択: {}", formula);
            continue;
        }

        if choice == 9 {
            formula.clear();//変数内をすべて消す？
            println!("選択: {}", formula);
            continue;
        }

        formula.push_str(symbols[choice - 1]);

        println!("選択: {}", formula);
    }
}