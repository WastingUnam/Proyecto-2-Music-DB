use super::ast::{Campo, Expr};

/// Resultado de convertir una expresión a SQL
pub struct QuerySql {
    pub where_clause: String,
    pub params: Vec<String>,
}

/// Convierte un AST a una cláusula WHERE de SQL con parámetros
pub fn expr_a_sql(expr: &Expr) -> QuerySql {
    let mut params = Vec::new();
    let where_clause = construir(expr, &mut params);
    QuerySql {
        where_clause,
        params,
    }
}

fn construir(expr: &Expr, params: &mut Vec<String>) -> String {
    match expr {
        Expr::Condicion { campo, valor } => {
            match campo {
                Campo::Year | Campo::Track => {
                    params.push(valor.clone());
                    let idx = params.len();
                    let columna = match campo {
                        Campo::Year => "r.year",
                        Campo::Track => "r.track",
                        _ => unreachable!(),
                    };
                    format!("CAST({} AS TEXT) = ?{}", columna, idx)
                }
                Campo::Performer => {
                    // Busca por nombre del performer directo,
                    // Y también por miembros del grupo (via in_group)
                    let like = format!("%{}%", valor);
                    params.push(like.clone());
                    let idx1 = params.len();
                    params.push(like);
                    let idx2 = params.len();
                    format!(
                        "(p.name LIKE ?{} OR r.id_performer IN (\
                            SELECT ig.id_group FROM in_group ig \
                            JOIN performers miembro ON ig.id_person = miembro.id_performer \
                            WHERE miembro.name LIKE ?{}\
                        ))",
                        idx1, idx2
                    )
                }
                _ => {
                    params.push(format!("%{}%", valor));
                    let idx = params.len();
                    let columna = match campo {
                        Campo::Title => "r.title",
                        Campo::Album => "a.name",
                        Campo::Genre => "r.genre",
                        _ => unreachable!(),
                    };
                    format!("{} LIKE ?{}", columna, idx)
                }
            }
        }
        Expr::And(izq, der) => {
            let sql_izq = construir(izq, params);
            let sql_der = construir(der, params);
            format!("({} AND {})", sql_izq, sql_der)
        }
        Expr::Or(izq, der) => {
            let sql_izq = construir(izq, params);
            let sql_der = construir(der, params);
            format!("({} OR {})", sql_izq, sql_der)
        }
        Expr::Not(inner) => {
            let sql_inner = construir(inner, params);
            format!("NOT ({})", sql_inner)
        }
    }
}
