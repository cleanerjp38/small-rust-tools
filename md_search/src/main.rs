use std::fs;
use std::env;

fn main() {

    //env::args()はどういう操作なんだろう？ターミナル系の操作なのかな
    let args: Vec<String> = env::args().collect();//cargo run -- testと入力すると、argsに要素が詰まった。
    //rust_analyzerでrunしたら、ターミナル操作ではなかったので、testは出力されなかった。
    println!("{:?}", args);//["target\\debug\\md_search.exe", "test"]と出力された

    let target_dir = &args[1];//この変数に検索先フォルダのPathを入れると、そこに飛べる。変更010にて、コマンドライン引数を入れた
    println!("検索先フォルダ:{}", target_dir);
    let entries = fs::read_dir(target_dir).unwrap();
    println!("{:?}", entries);//これだとReadDir(".")が出力された
    
    let keyword = &args[2];//コマンドライン引数で、Markdown内の検索ワードを変数に入れる

    for entry in entries {
        //println!("{:?}", entry);//Ok(DirEntry(".\\.git"))
        let entry = entry.unwrap();
        //println!("{:?}", entry);//DirEntry(".\\.git")

        let path = entry.path();
        //println!("{:?}", path);//".\\.git"
        //println!("{:?}", path.extension());//None
        if path.extension() == Some(std::ffi::OsStr::new("md")) {
            let content = fs::read_to_string(&path).unwrap();

                if content.contains(keyword){
                println!("「{}」が見つかりました！ {:?}", keyword, path.file_name());
            }// else { 見つからない場合は出力しないようにしないと、Markdownが膨大な場合はメッセージ量が酷いことになる
                //println!("「{}」は見つかりませんでした", keyword);
            //}
        }
    }
}