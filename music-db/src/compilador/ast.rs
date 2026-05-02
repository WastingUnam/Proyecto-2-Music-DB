/// Campos por los que se puede buscar
#[derive(Debug, Clone, PartialEq)]
pub enum Campo {
    Title,
    Album,
    Performer,
    Year,
    Genre,
    Track,
}

/// Árbol de expresiones para la búsqueda
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// campo:"valor"
    Condicion { campo: Campo, valor: String },
    /// expr ^ expr
    And(Box<Expr>, Box<Expr>),
    /// expr + expr
    Or(Box<Expr>, Box<Expr>),
    /// ~expr
    Not(Box<Expr>),
}
