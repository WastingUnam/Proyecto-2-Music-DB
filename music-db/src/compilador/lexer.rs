use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "compilador/search.pest"]
struct SearchLexer;

/// Tokens que produce el lexer para LALRPOP
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    And,
    Or,
    Not,
    LParen,
    RParen,
    Colon,
    Campo(String),
    Valor(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::And => write!(f, "^"),
            Token::Or => write!(f, "+"),
            Token::Not => write!(f, "~"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Colon => write!(f, ":"),
            Token::Campo(s) => write!(f, "{}", s),
            Token::Valor(s) => write!(f, "\"{}\"", s),
        }
    }
}

/// Tipo que espera LALRPOP: (posición_inicio, token, posición_fin)
pub type Spanned = (usize, Token, usize);

/// Tokeniza el input usando Pest y produce tokens para LALRPOP
pub fn tokenizar(input: &str) -> Result<Vec<Spanned>, String> {
    let pairs = SearchLexer::parse(Rule::tokens, input)
        .map_err(|e| format!("Error léxico: {}", e))?;

    let mut tokens = Vec::new();

    for pair in pairs.into_iter().next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::token => {
                let inner = pair.into_inner().next().unwrap();
                let inner_start = inner.as_span().start();
                let inner_end = inner.as_span().end();

                let tok = match inner.as_rule() {
                    Rule::and_op => Token::And,
                    Rule::or_op => Token::Or,
                    Rule::not_op => Token::Not,
                    Rule::lparen => Token::LParen,
                    Rule::rparen => Token::RParen,
                    Rule::colon => Token::Colon,
                    Rule::campo => Token::Campo(inner.as_str().to_string()),
                    Rule::valor_comillas => {
                        let contenido = inner
                            .into_inner()
                            .next()
                            .map(|p| p.as_str().to_string())
                            .unwrap_or_default();
                        Token::Valor(contenido)
                    }
                    Rule::valor_simple => Token::Valor(inner.as_str().to_string()),
                    _ => continue,
                };
                tokens.push((inner_start, tok, inner_end));
            }
            Rule::EOI => {}
            _ => {}
        }
    }

    Ok(tokens)
}
