// src/main.rs
mod instruction; // instruction.rsを読み込む
mod parser;
use instruction::Instruction; // 使いやすいようにインポート
use parser::Parser;

fn main() {
    println!("Hello, world!");
    let parser = Parser::new("/Users/yato/Developer/Rust/nand2tetris-rust/Add.asm");
    println!("{:?}",parser.command_str_list);
    println!("{:?}",parser.has_more_lines());
}
