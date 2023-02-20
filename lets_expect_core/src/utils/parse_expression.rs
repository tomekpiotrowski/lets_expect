use syn::{parenthesized, parse::ParseBuffer, Error, Expr, Token};

pub struct ExpectationExpression {
    pub(crate) mutable: bool,
    pub(crate) reference: bool,
    pub(crate) expr: Expr,
}

pub fn parse_expectation_expression(input: &ParseBuffer) -> Result<ExpectationExpression, Error> {
    let content;
    parenthesized!(content in input);

    let mut reference = false;
    if content.peek(Token![&]) {
        content.parse::<Token![&]>()?;
        reference = true;
    }

    let mut mutable = false;
    if content.peek(Token![mut]) {
        content.parse::<Token![mut]>()?;
        mutable = true;
    }

    let expr = content.parse::<Expr>()?;

    Ok(ExpectationExpression {
        mutable,
        reference,
        expr,
    })
}

pub fn parse_expr(input: &ParseBuffer) -> Result<Expr, Error> {
    let content;
    parenthesized!(content in input);

    let expr = content.parse::<Expr>()?;
    Ok(expr)
}
