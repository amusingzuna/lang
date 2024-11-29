#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Type {
    Primitive(String),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Literal {
    Float(String),
    Integer(String),
    Reference(String),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expression {
    Block(Vec<Statement>),
    Literal(Literal),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Statement {
    Declare(Option<Type>, String),
    Assignment(String, Expression),
    Instantiate(Option<Type>, String, Expression),
    NoOp,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Program(pub Vec<Statement>);
