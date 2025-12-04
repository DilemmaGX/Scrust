#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Variable(VariableDecl),
    Costume(AssetDecl),
    Sound(AssetDecl),
    Function(Function),
    Procedure(ProcedureDef), // Custom block definition
    Comment(String),         // Top level comment
    BatchBreak,              // Separator for block batches (blank lines)
    Stmt(Stmt),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    Default,
}

#[derive(Debug, Clone)]
pub struct ProcedureDef {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
    pub is_warp: bool, // Run without screen refresh
    pub comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VariableDecl {
    pub name: String,
    pub ty: Type,
    pub init: Expr,
    pub visibility: Visibility,
    pub comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AssetDecl {
    pub name: String,
    pub path: String,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Function {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
    pub is_warp: bool,
    pub comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    #[allow(dead_code)]
    VarDecl(String, Expr, Option<String>),
    Assign(String, Expr, Option<String>),
    Expr(Expr, Option<String>),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>, Option<String>),
    Repeat(Expr, Vec<Stmt>, Option<String>),
    Forever(Vec<Stmt>, Option<String>),
    Until(Expr, Vec<Stmt>, Option<String>),
    Match(
        Expr,
        Vec<(Expr, Vec<Stmt>)>,
        Option<Vec<Stmt>>,
        Option<String>,
    ),
    #[allow(dead_code)]
    Return(Option<Expr>, Option<String>),
    CBlock(String, Vec<Expr>, Vec<Stmt>, Option<String>),
    Comment(String),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Bool(bool),
    Variable(String),
    Call(String, Vec<Expr>),
    #[allow(dead_code)]
    ProcCall(String, Vec<Expr>),
    BinOp(Box<Expr>, Op, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
    List(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum UnOp {
    Not,
    Neg,
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Gt,
    Lt,
    Ne,
    Ge,
    Le,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    String,
    Boolean,
    List,
    Unknown,
}
