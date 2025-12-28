pub enum Instruction {
    A(String),
    C {
        dest: Option<String>,
        comp: String,
        jump: Option<String>,
    },
    L(String),
}