use std::fs;

fn main() {
    let content = fs::read_to_string("Cargo.toml").unwrap();

    let keyword = "workspace";

    if content.contains(keyword){
        println!("「{}」が見つかりました！", keyword);
    } else {
        println!("「{}」は見つかりませんでした", keyword);
    }

    let entries = fs::read_dir(".").unwrap();
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
        }
    }
}