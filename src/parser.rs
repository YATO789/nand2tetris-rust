use std::fs;

// instruction.rs で定義されていると想定
#[derive(Debug, PartialEq)]
pub enum Instruction {
    AInstruction, // @Xxx
    CInstruction, // dest=comp;jump
    LInstruction, // (LABEL)
}

pub struct Parser {
    pub current_command_idx: usize,
    pub command_str_list: Vec<String>,
}

impl Parser {
    pub fn new(file_path: &str) -> Self {
        match fs::read_to_string(file_path) {
            Ok(content) => {
                let lines: Vec<String> = content
                    .lines()
                    .map(|line| line.split("//").next().unwrap().trim().to_string())
                    .filter(|line| !line.is_empty())
                    .collect();
                Self {
                    command_str_list: lines,
                    current_command_idx: 0,
                }
            }
            Err(e) => {
                eprintln!("読み込みエラー: {}", e);
                Self {
                    command_str_list: vec![],
                    current_command_idx: 0,
                }
            }
        }
    }

    pub fn has_more_lines(&self) -> bool {
        self.current_command_idx < self.command_str_list.len()
    }

    pub fn advance(&mut self) {
        if self.has_more_lines() {
            self.current_command_idx += 1;
        }
    }

    fn get_current_command(&self) -> &String {
        &self.command_str_list[self.current_command_idx]
    }

    pub fn instruction_type(&self) -> Instruction {
        let command = self.get_current_command();
        if command.starts_with('@') {
            Instruction::AInstruction
        } else if command.starts_with('(') && command.ends_with(')') {
            Instruction::LInstruction
        } else {
            Instruction::CInstruction
        }
    }

    pub fn symbol(&self) -> String {
        let command = self.get_current_command();
        if command.starts_with('@') {
            command[1..].to_string()
        } else if command.starts_with('(') && command.ends_with(')') {
            command[1..command.len() - 1].to_string()
        } else {
            "".to_string()
        }
    }

    /// C命令のdest部分を返す
    pub fn dest(&self) -> String {
        let command = self.get_current_command();
        if command.contains('=') {
            command.split('=').next().unwrap_or("").to_string()
        } else {
            "null".to_string() // destがない場合はnull
        }
    }

    /// C命令のcomp部分を返す (最重要)
    pub fn comp(&self) -> String {
        let command = self.get_current_command();
        // 1. destがあれば取り除く
        let temp = if command.contains('=') {
            command.split('=').nth(1).unwrap_or("")
        } else {
            command.as_str()
        };
        // 2. jumpがあれば取り除く
        temp.split(';').next().unwrap_or("").to_string()
    }

    /// C命令のjump部分を返す
    pub fn jump(&self) -> String {
        let command = self.get_current_command();
        if command.contains(';') {
            command.split(';').nth(1).unwrap_or("").to_string()
        } else {
            "null".to_string() // jumpがない場合はnull
        }
    }
}