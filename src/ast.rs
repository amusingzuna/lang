#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Type {
    Primitive(String),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Statement {
    Declare(Type, String),
    NoOp,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Program(pub Vec<Statement>);
