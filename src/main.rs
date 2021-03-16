use std::env;
use walkdir::WalkDir;
use std::path::Path;

mod modules;

use crate::modules::conv::decode_filename;

/// まいんちゃん
fn main() {
    let path;
    {
        let args: Vec<String> = env::args().collect();
        path = args[1].clone();
    }
    println!("SEARCH: {}", path);
    
    // サブディレクトリを再起検索
    // ディレクトリはスキップ
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        process(&entry.path())
    }
}

/// 1ファイルごとの処理
/// %xxでエンコードされているファイル名をデコードしてリネームする
/// (今は怖いのでコピーしている)
fn process(path: &Path)
{
    let filename = path.file_name().unwrap().to_str().unwrap();
    let new_name = decode_filename(filename);
    if filename != new_name {
        let parent = path.parent().unwrap();
        let src = parent.join(filename);
        let dst = parent.join(new_name);
        print!("{:?} -> ", src);
        println!("{:?}", dst);
        match std::fs::copy(src, dst) {
            Ok(_) => {},
            Err(err) => println!("ERROR: {}", err)
        }
    }
}
