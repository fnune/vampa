type Identifier = String;

pub struct Module {
    pub file_name: String,
    pub statements: Vec<Box<Statement>>,
}

pub struct Statement {
    pub kind: StatementKind,
}

pub enum StatementKind {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    /// An expression statement without a semicolon, expected to be implicitly returned.
    Return(Box<Expression>),
}

pub struct VariableDeclaration {
    pub name: Box<Identifier>,
    pub typ: Box<Typ>,
    pub value: Box<Expression>,
}

pub struct FunctionDeclaration {
    pub name: Box<Identifier>,
    pub parameters: Vec<Box<Parameter>>,
    pub return_typ: Option<Box<Typ>>,
    pub body: Box<Expression>,
}

pub struct Parameter {
    pub name: Box<Identifier>,
    pub typ: Box<Typ>,
}

pub struct Expression {
    pub kind: ExpressionKind,
}

pub enum ExpressionKind {
    /// A literal expression such as `42`.
    Literal(Literal),
    /// A function call such as `sum 20 10` FunctionCall(F, P) where F is the function expression
    /// itself and P is the Vector of parameters.
    FunctionCall(Box<Expression>, Vec<Box<Expression>>),
    /// A binary operation such as `+ 20 20`
    BinaryOperation(BinaryOperation, Box<Expression>, Box<Expression>),
    /// Blocks always evaluate to something, so they're also expressions.
    Block(Box<Block>),
    /// A reference to a variable.
    VariableReference(Box<Identifier>),
}

pub struct Block {
    /// Must end with a `Statement` of `StatementKind::Return`.
    pub statements: Vec<Box<Statement>>,
}

/// Even though Vampa uses prefix notation, we still refer
/// to binary operands as <operator> <left> <right>.
pub struct BinaryOperation {
    pub kind: BinaryOperationKind,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

/// Should eventually support other commonly expected binary operations.
pub enum BinaryOperationKind {
    Addition,
}

pub struct Literal {
    pub kind: LiteralKind,
}

pub enum LiteralKind {
    Int(i128, LiteralIntType),
}

/// Should eventually support signed and unsigned as well.
pub enum LiteralIntType {
    Unsuffixed,
}

pub struct Typ {
    pub kind: TypKind,
}

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
pub enum IntType {
    I32,
}
