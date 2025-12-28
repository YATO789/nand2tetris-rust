use crate::instruction::Instruction;
use std::fs;

//行ごとの文字列を読み込み、不要な空白やコメントを除去し、1行ずつアクセスできるように
//記号命令をその基礎となる構成要素に分解
pub struct Parser {
   pub current_command_idx: usize,
   pub  command_str_list: Vec<String>,
}

impl Parser {
    pub fn new(file_path: &str) -> Self{
        match fs::read_to_string(file_path) {
            Ok(content) => {
                println!("ファイルの内容:\n{}", content);

                //コメントと空白を除去
                let lines: Vec<String> = content
                .lines()
                .map(|line| line.split("//").next().unwrap().trim().to_string())
                .filter(|line| !line.is_empty())
                .collect();
                Self{
                    command_str_list: lines,
                    current_command_idx:0,
                }
            }
            Err(e) => {
                eprintln!("読み込みエラー: {}", e);
                Self {
                    command_str_list: vec![],
                    current_command_idx:0
                }
            }
        }
    }


}