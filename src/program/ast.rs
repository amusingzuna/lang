#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Type {
    Primitive(String),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Literal {
    Float(String),
    Integer(String),
    String(String),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expression {
    Literal(Literal),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Statement {
    Declare(Type, String),
    Assignment(String, Expression),
    Instantiate(Type, String, Expression),
    NoOp,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Program(pub Vec<Statement>);
