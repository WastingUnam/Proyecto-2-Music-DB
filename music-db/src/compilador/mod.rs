pub mod ast;
pub mod lexer;
pub mod query_builder;

lalrpop_util::lalrpop_mod!(
    #[allow(clippy::all, unused)]
    parser,
    "/compilador/parser.rs"
);

use ast::Expr;
use lexer::tokenizar;

/// Parsea una query de búsqueda y regresa el AST
pub fn parsear(input: &str) -> Result<Expr, String> {
    let tokens = tokenizar(input)?;
    let parser = parser::ExprParser::new();
    parser
        .parse(tokens.into_iter())
        .map_err(|e| format!("Error de sintaxis: {}", e))
}
