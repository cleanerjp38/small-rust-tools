use std::fs;

fn main() {
    let content = fs::read_to_string("Cargo.toml").unwrap();

    let keyword = "workspace";

    if content.contains(keyword){
        println!("「{}」が見つかりました！", keyword);
    } else {
        println!("「{}」は見つかりませんでした", keyword);
    }
}