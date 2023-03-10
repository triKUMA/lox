use super::scanner::*;

pub enum Expr<'a> {
    Value(TokenValue<'a>),
    Grouping(BoxedExpr<'a>),
    Unary(Token<'a>, BoxedExpr<'a>),
    Binary(BoxedExpr<'a>, Token<'a>, BoxedExpr<'a>),
}

type BoxedExpr<'a> = Box<Expr<'a>>;

impl<'a> std::fmt::Display for Expr<'a> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Value(val) => match val {
                TokenValue::String(string) => {
                    let string = format!("\"{}\"", string);
                    formatter.write_str(string.as_ref())
                }
                TokenValue::Number(num) => {
                    let string = num.to_string();
                    formatter.write_str(string.as_ref())
                }
                TokenValue::None => formatter.write_str("null"),
            },
            Expr::Grouping(expr) => {
                let string = format!("(group {})", expr);
                formatter.write_str(string.as_ref())
            }
            Expr::Unary(token, expr) => {
                let string = format!("({}{})", token, expr);
                formatter.write_str(string.as_ref())
            }
            Expr::Binary(expr1, token, expr2) => {
                let string = format!("({} {} {})", expr1, token, expr2);
                formatter.write_str(string.as_ref())
            }
        }
    }
}
