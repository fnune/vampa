type Identifier = String;

#[derive(Debug, PartialEq)]
pub struct Module {
    pub file_name: String,
    pub statements: Vec<Box<Statement>>,
}

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub kind: StatementKind,
}

#[derive(Debug, PartialEq)]
pub enum StatementKind {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    /// An expression statement expected to be implicitly returned.
    Return(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    pub name: Box<Identifier>,
    pub typ: Box<Typ>,
    pub value: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Box<Identifier>,
    pub parameters: Parameters,
    pub return_typ: Box<Typ>,
    pub body: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: Box<Identifier>,
    pub typ: Box<Typ>,
}

pub type Parameters = Vec<Box<Parameter>>;

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
}

#[derive(Debug, PartialEq)]
pub enum ExpressionKind {
    /// A literal expression such as `42`.
    Literal(Literal),
    /// A function call such as `sum 20 10` FunctionCall(F, P) where F is the
    /// function expression itself and P is the Vector of parameters.
    FunctionCall(Box<Expression>, Vec<Box<Expression>>),
    /// A binary operation such as `+ 20 20`
    BinaryOperation(BinaryOperation),
    /// Blocks always evaluate to something, so they're also expressions.
    Block(Box<Block>),
    /// A reference to a variable.
    VariableReference(Box<Identifier>),
}

#[derive(Debug, PartialEq)]
pub struct Block {
    /// Must end with a `Statement` of `StatementKind::Return`.
    pub statements: Vec<Box<Statement>>,
}

/// Even though Vampa uses prefix notation, we still refer
/// to binary operands as <operator> <left> <right>.
#[derive(Debug, PartialEq)]
pub struct BinaryOperation {
    pub kind: BinaryOperationKind,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

/// Should eventually support other commonly expected binary operations.
#[derive(Debug, PartialEq)]
pub enum BinaryOperationKind {
    Addition,
}

#[derive(Debug, PartialEq)]
pub struct Literal {
    pub kind: LiteralKind,
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    Int(i128, LiteralIntType),
}

/// Should eventually support signed and unsigned as well.
#[derive(Debug, PartialEq)]
pub enum LiteralIntType {
    Unsuffixed,
}

#[derive(Debug, PartialEq)]
pub struct Typ {
    pub kind: TypKind,
}

#[derive(Debug, PartialEq)]
pub enum TypKind {
    Int(IntType),
    /// A placeholder for a type that must be inferred.
    Infer,
}

impl Typ {
    pub fn infer() -> Typ {
        Typ {
            kind: { TypKind::Infer },
        }
    }
}

/// Should eventually support the types advertised in the README.
#[derive(Debug, PartialEq)]
pub enum IntType {
    I32,
}
