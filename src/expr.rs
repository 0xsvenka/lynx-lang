#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    TyCon(String),
    TyVar(String),
    TyApp(Box<Type>, Box<Type>),
    TyTuple(Vec<Type>),
    TyList(Box<Type>),
    TyFun(Box<Type>, Box<Type>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Constructor {
    pub name: String,
    pub args: Vec<Type>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataDecl {
    pub name: String,
    pub type_vars: Vec<String>,
    pub constructors: Vec<Constructor>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDecl {
    pub name: String,
    pub type_sig: Option<(Vec<String>, Type)>,
    pub equations: Vec<Equation>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Equation {
    pub lhs: Pattern,
    pub rhs: Expr,
    pub where_clause: Option<Vec<FunctionDecl>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    PVar(String),
    PLit(Literal),
    PCon(String, Vec<Pattern>),
    PWildcard,
    PTuple(Vec<Pattern>),
    PList(Vec<Pattern>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Var(String),
    Lit(Literal),
    App(Box<Expr>, Box<Expr>),
    Lambda(Vec<Pattern>, Box<Expr>),
    Let(Vec<FunctionDecl>, Box<Expr>),
    Case(Box<Expr>, Vec<Alt>),
    Tuple(Vec<Expr>),
    List(Vec<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Alt {
    pub pat: Pattern,
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Module {
    pub name: String,
    pub imports: Vec<Import>,
    pub decls: Vec<Decl>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub module: String,
    pub qualified: bool,
    pub as_name: Option<String>,
    pub imports: Option<Vec<ImportSpec>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImportSpec {
    Function(String),
    Type(String),
    All,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Decl {
    Data(DataDecl),
    Function(FunctionDecl),
    TypeSig(String, Type),
}
