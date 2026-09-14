use std::fs;
use std::env;

fn main() {
    let content = fs::read_to_string("Cargo.toml").unwrap();

    let keyword = "workspace";

    if content.contains(keyword){
        println!("「{}」が見つかりました！", keyword);
    } else {
        println!("「{}」は見つかりませんでした", keyword);
    }

    //env::args()はどういう操作なんだろう？ターミナル系の操作なのかな
    let args: Vec<String> = env::args().collect();//cargo run -- testと入力すると、argsに要素が詰まった。
    //rust_analyzerでrunしたら、ターミナル操作ではなかったので、testは出力されなかった。
    println!("{:?}", args);//["target\\debug\\md_search.exe", "test"]と出力された

    let target_dir = ".";//この変数に検索先フォルダのPathを入れると、そこに飛べる
    println!("検索先フォルダ:{}", target_dir);
    let entries = fs::read_dir(target_dir).unwrap();
    println!("{:?}", entries);//これだとReadDir(".")が出力された

    for entry in entries {
        println!("{:?}", entry);//Ok(DirEntry(".\\.git"))
        let entry = entry.unwrap();
        println!("{:?}", entry);//DirEntry(".\\.git")

        let path = entry.path();
        println!("{:?}", path);//".\\.git"
        println!("{:?}", path.extension());//None
        if path.extension() == Some(std::ffi::OsStr::new("md")) {
            println!("Markdown発見: {:?}", path);//現在、md_searchフォルダにmdファイルはないので、なにも表示されない
            println!("ファイル名: {:?}", path.file_name());//もしmdファイルがあれば、名前を表示する
        }
    }
}