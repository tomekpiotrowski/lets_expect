use syn::{parenthesized, parse::ParseBuffer, Error, Expr, Token};

pub fn parse_expr_with_mutable(input: &ParseBuffer) -> Result<(bool, Expr), Error> {
    let content;
    parenthesized!(content in input);

    let mut mutable = false;

    if content.peek(Token![mut]) {
        content.parse::<Token![mut]>()?;
        mutable = true;
    }

    let expr = content.parse::<Expr>()?;
    Ok((mutable, expr))
}

pub fn parse_expr(input: &ParseBuffer) -> Result<Expr, Error> {
    let content;
    parenthesized!(content in input);

    let expr = content.parse::<Expr>()?;
    Ok(expr)
}
