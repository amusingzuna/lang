#[derive(Debug, Clone)]
pub enum Statement {
    NoOp,
}

#[derive(Debug)]
pub struct Program(pub Vec<Statement>);
