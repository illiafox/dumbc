pub enum Expr {
    Const(i32),
}

pub enum Stmt {
    Return(Expr),
}

pub struct Function {
    pub name: String,
    pub body: Stmt,
}

pub struct Program {
    pub function: Function,
}
