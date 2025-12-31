mod parser;
mod code;
mod symbol_table;

use parser::{Instruction, Parser};
use code::Code;
use symbol_table::SymbolTable;
use std::fs::File;
use std::io::Write;

fn main() {
    let input_path = "/Users/yato/Developer/Rust/nand2tetris-rust/Add.asm";
    let output_path = input_path.replace(".asm", ".hack");

    let mut symbol_table = SymbolTable::new();
    let mut parser = Parser::new(input_path);

    // --- Pass 1: ラベル収集 ---
    let mut rom_address = 0;
    while parser.has_more_lines() {
        match parser.instruction_type() {
            Instruction::LInstruction => {
                symbol_table.add_entry(parser.symbol(), rom_address);
            }
            _ => rom_address += 1,
        }
        parser.advance();
    }

    // --- Pass 2: 翻訳 ---
    parser.reset();
    let mut machine_code_list = Vec::new();
    let mut next_variable_address = 16;

    while parser.has_more_lines() {
        match parser.instruction_type() {
            Instruction::AInstruction => {
                let symbol = parser.symbol();
                let address = match symbol.parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        if !symbol_table.contains(&symbol) {
                            symbol_table.add_entry(symbol.clone(), next_variable_address);
                            next_variable_address += 1;
                        }
                        symbol_table.get(&symbol).unwrap()
                    }
                };
                machine_code_list.push(format!("{:016b}", address));
            }
            Instruction::CInstruction => {
                let binary = format!(
                    "111{}{}{}",
                    Code::comp(&parser.comp()),
                    Code::dest(&parser.dest()),
                    Code::jump(&parser.jump())
                );
                machine_code_list.push(binary);
            }
            Instruction::LInstruction => {} // ラベル行は出力しない
        }
        parser.advance();
    }

    // --- ファイル出力 ---
    let mut file = File::create(output_path).expect("出力ファイルの作成に失敗しました");
    for line in machine_code_list {
        writeln!(file, "{}", line).expect("書き込みに失敗しました");
    }

    println!("アセンブル完了！ .hack ファイルを生成しました。");
}