use std::fs;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    AInstruction,
    CInstruction,
    LInstruction,
}

pub struct Parser {
    pub current_command_idx: usize,
    pub command_str_list: Vec<String>,
}

impl Parser {
    pub fn new(file_path: &str) -> Self {
        let content = fs::read_to_string(file_path).expect("Failed to read file");
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

    pub fn has_more_lines(&self) -> bool {
        self.current_command_idx < self.command_str_list.len()
    }

    pub fn advance(&mut self) {
        self.current_command_idx += 1;
    }

    pub fn reset(&mut self) {
        self.current_command_idx = 0;
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
        } else if command.starts_with('(') {
            command[1..command.len() - 1].to_string()
        } else {
            "".to_string()
        }
    }

    pub fn dest(&self) -> String {
        let command = self.get_current_command();
        if let Some(pos) = command.find('=') {
            command[..pos].to_string()
        } else {
            "".to_string()
        }
    }

    pub fn comp(&self) -> String {
        let command = self.get_current_command();
        let temp = if let Some(pos) = command.find('=') {
            &command[pos + 1..]
        } else {
            command
        };
        temp.split(';').next().unwrap_or("").to_string()
    }

    pub fn jump(&self) -> String {
        let command = self.get_current_command();
        if let Some(pos) = command.find(';') {
            command[pos + 1..].to_string()
        } else {
            "".to_string()
        }
    }
}