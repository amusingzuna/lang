#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Type {
    Atomic(String),
    Array(Box<Type>),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Literal {
    Float(String),
    Integer(String),
    Boolean(bool),
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
    Assign(String, Expression),
    Instantiate(Option<Type>, String, Expression),
    Expression(Expression),
    NoOp,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Program(pub Vec<Statement>);
