use english_numbers::{convert, Formatting};
use proc_macro2::Ident;
use syn::{
    punctuated::Punctuated, token::Comma, BinOp, Expr, ExprAssign, ExprRange, Lit, Local, Member,
    Pat, Stmt, Type, UnOp,
};

pub fn stmt_to_ident(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Local(local) => local_to_ident(local),
        Stmt::Item(_) => unimplemented!("Item not supported"),
        Stmt::Expr(expr) => expr_to_ident(expr),
        Stmt::Semi(_, _) => unimplemented!("Semi not supported"),
    }
}

pub fn local_to_ident(local: &Local) -> String {
    let ident = pat_to_ident(&local.pat).to_string();
    ident
        + "_is_"
        + expr_to_ident(
            &local
                .init
                .as_ref()
                .expect("When `let` statements are expected to have an initial value")
                .1,
        )
        .as_str()
}

pub fn pat_to_ident(pat: &Pat) -> Ident {
    match pat {
        Pat::Ident(pat) => pat.ident.clone(),
        Pat::Type(pat) => pat_to_ident(&pat.pat),
        _ => unimplemented!("Unable to convert pattern to identifier"),
    }
}

pub fn expr_to_ident(expr: &syn::Expr) -> String {
    match expr {
        Expr::Array(array) => {
            if array.elems.is_empty() {
                "empty_array".to_string()
            } else {
                punctuated_to_ident(&array.elems)
            }
        }
        Expr::Assign(assign) => expr_assign_to_ident(assign),
        Expr::AssignOp(assign_op) => {
            expr_to_ident(&assign_op.left)
                + "_"
                + binary_op_to_ident(&assign_op.op)
                + "_"
                + &expr_to_ident(&assign_op.right)
        }
        Expr::Async(_) => unimplemented!("Async not supported"),
        Expr::Await(expr_await) => "await_".to_string() + &expr_to_ident(&expr_await.base),
        Expr::Binary(binary) => {
            expr_to_ident(&binary.left)
                + "_"
                + binary_op_to_ident(&binary.op)
                + "_"
                + &expr_to_ident(&binary.right)
        }
        Expr::Block(block) => block_to_ident(block),
        Expr::Box(boxed) => "boxed_".to_string() + &expr_to_ident(&boxed.expr),
        Expr::Break(_) => unimplemented!("Break not supported"),
        Expr::Call(call) => {
            expr_to_ident(&call.func)
                + if !call.args.is_empty() {
                    "_".to_string() + punctuated_to_ident(&call.args).as_str()
                } else {
                    "".to_string()
                }
                .as_str()
        }
        Expr::Cast(cast) => expr_to_ident(&cast.expr) + "_as_" + &type_to_ident(&cast.ty),
        Expr::Closure(_) => unimplemented!("Closure not supported"),
        Expr::Continue(_) => unimplemented!("Continue not supported"),
        Expr::Field(field) => {
            expr_to_ident(&field.base) + "_" + member_to_ident(&field.member).as_str()
        }
        Expr::ForLoop(_) => unimplemented!("ForLoop not supported"),
        Expr::Group(_) => unimplemented!("Group not supported"),
        Expr::If(if_expr) => "if_".to_string() + &expr_to_ident(&if_expr.cond),
        Expr::Index(index) => format!(
            "{}_at_{}",
            expr_to_ident(&index.expr),
            expr_to_ident(&index.index)
        ),
        Expr::Let(_) => unimplemented!("Let not supported"),
        Expr::Lit(lit) => expr_lit_to_ident(lit),
        Expr::Loop(_) => unimplemented!("Loop not supported"),
        Expr::Macro(mac) => mac.mac.path.get_ident().unwrap().to_string().to_lowercase(),
        Expr::Match(_) => "match".to_string(),
        Expr::MethodCall(method_call) => {
            expr_to_ident(&method_call.receiver)
                + "_"
                + method_call.method.to_string().to_lowercase().as_str()
                + if !method_call.args.is_empty() {
                    "_".to_string() + punctuated_to_ident(&method_call.args).as_str()
                } else {
                    "".to_string()
                }
                .as_str()
        }
        Expr::Paren(paren) => expr_to_ident(&paren.expr),
        Expr::Path(path) => path_to_ident(path),
        Expr::Range(range) => range_to_ident(range),
        Expr::Reference(reference) => expr_to_ident(&reference.expr),
        Expr::Repeat(_) => unimplemented!("Repeat not supported"),
        Expr::Return(_) => unimplemented!("Return not supported"),
        Expr::Struct(struc) => struc.path.get_ident().unwrap().to_string().to_lowercase(),
        Expr::Try(_) => unimplemented!("Try not supported"),
        Expr::TryBlock(_) => unimplemented!("TryBlock not supported"),
        Expr::Tuple(tuple) => punctuated_to_ident(&tuple.elems),
        Expr::Type(_) => unimplemented!("Type not supported"),
        Expr::Unary(unary) => {
            unary_op_to_ident(&unary.op).to_string() + "_" + &expr_to_ident(&unary.expr)
        }
        Expr::Unsafe(_) => unimplemented!("Unsafe not supported"),
        Expr::Verbatim(_) => unimplemented!("Verbatim not supported"),
        Expr::While(_) => unimplemented!("While not supported"),
        Expr::Yield(_) => unimplemented!("Yield not supported"),
        _ => unimplemented!("Expected Expr, got {:?}", expr),
    }
}

fn type_to_ident(ty: &Type) -> String {
    match ty {
        Type::Array(array) => format!(
            "array_of_{}_{}",
            expr_to_ident(&array.len),
            type_to_ident(&array.elem)
        ),
        Type::BareFn(_) => unimplemented!(),
        Type::Group(_) => unimplemented!(),
        Type::ImplTrait(_) => unimplemented!(),
        Type::Infer(_) => unimplemented!(),
        Type::Macro(_) => unimplemented!(),
        Type::Never(_) => unimplemented!(),
        Type::Paren(_) => unimplemented!(),
        Type::Path(path) => path
            .path
            .segments
            .last()
            .unwrap()
            .ident
            .to_string()
            .to_lowercase(),
        Type::Ptr(_) => unimplemented!(),
        Type::Reference(_) => unimplemented!(),
        Type::Slice(_) => unimplemented!(),
        Type::TraitObject(_) => unimplemented!(),
        Type::Tuple(_) => unimplemented!(),
        Type::Verbatim(_) => unimplemented!(),
        _ => unimplemented!(),
    }
}

pub fn path_to_ident(path: &syn::ExprPath) -> String {
    path.path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string().to_lowercase())
        .collect::<Vec<String>>()
        .join("_")
}

fn block_to_ident(block: &syn::ExprBlock) -> String {
    block
        .block
        .stmts
        .last()
        .map(stmt_to_ident)
        .unwrap_or_else(|| "noop".to_string())
}

fn member_to_ident(member: &Member) -> String {
    match member {
        Member::Named(named) => named.to_string(),
        Member::Unnamed(unnamed) => unnamed.index.to_string(),
    }
}

fn range_to_ident(range: &ExprRange) -> String {
    let from = range
        .from
        .as_ref()
        .map(|expr| format!("_from_{}", expr_to_ident(expr)))
        .unwrap_or_default();
    let to = range
        .to
        .as_ref()
        .map(|expr| format!("_to_{}", expr_to_ident(expr)))
        .unwrap_or_default();
    format!("range{}{}", from, to)
}

fn expr_assign_to_ident(assign: &ExprAssign) -> String {
    expr_to_ident(&assign.left) + "_is_" + &expr_to_ident(&assign.right)
}

pub fn punctuated_to_ident(punctuated: &Punctuated<Expr, Comma>) -> String {
    punctuated
        .iter()
        .map(expr_to_ident)
        .collect::<Vec<String>>()
        .join("_")
}

// pub fn punctuated_assignments_to_ident(punctuated: &Punctuated<Assignment, Comma>) -> String {
//     punctuated.iter().map(|assignment| format!("{}_is_{}", assignment.name, expr_to_ident(&assignment.value))).collect::<Vec<String>>().join("_and_")
// }

fn expr_lit_to_ident(lit: &syn::ExprLit) -> String {
    match &lit.lit {
        Lit::Str(_) => "string".to_string(),
        Lit::ByteStr(_) => unimplemented!(),
        Lit::Byte(_) => unimplemented!(),
        Lit::Char(_) => unimplemented!(),
        Lit::Int(value) => {
            if let Ok(parsed) = value.base10_parse::<i64>() {
                humanize(parsed)
            } else {
                "number_".to_string() + value.to_string().as_str()
            }
        }
        Lit::Float(value) => {
            let value: f64 = value.base10_parse().unwrap();
            let formatted = format!("{:.2}", value);
            let parts = formatted.split('.').collect::<Vec<&str>>();
            let int_part = parts[0].parse::<i64>().unwrap();
            let fraction_part = parts[1].parse::<i64>().unwrap();
            let int_part = humanize(int_part);
            let fraction_part = humanize(fraction_part);

            format!("{}_point_{}", int_part, fraction_part)
        }
        Lit::Bool(value) => value.value.to_string(),
        Lit::Verbatim(_) => unimplemented!(),
    }
}

fn humanize(value: i64) -> String {
    convert(
        value,
        Formatting {
            dashes: false,
            title_case: false,
            ..Formatting::default()
        },
    )
}

fn unary_op_to_ident(op: &UnOp) -> &'static str {
    match op {
        UnOp::Deref(_) => "deref",
        UnOp::Not(_) => "not",
        UnOp::Neg(_) => "neg",
    }
}

fn binary_op_to_ident(op: &BinOp) -> &'static str {
    match op {
        BinOp::Add(_) => "plus",
        BinOp::Sub(_) => "minus",
        BinOp::Mul(_) => "times",
        BinOp::Div(_) => "divided_by",
        BinOp::Rem(_) => "remainder",
        BinOp::And(_) => "and",
        BinOp::Or(_) => "or",
        BinOp::BitXor(_) => "bit_xor",
        BinOp::BitAnd(_) => "bit_and",
        BinOp::BitOr(_) => "bit_or",
        BinOp::Shl(_) => "shift_left_by",
        BinOp::Shr(_) => "shift_right_by",
        BinOp::Eq(_) => "equals",
        BinOp::Lt(_) => "less_than",
        BinOp::Le(_) => "less_equal_than",
        BinOp::Ne(_) => "not_equal",
        BinOp::Ge(_) => "greater_equal_than",
        BinOp::Gt(_) => "greater_than",
        BinOp::AddEq(_) => "add_equal",
        BinOp::SubEq(_) => "subtract_equal",
        BinOp::MulEq(_) => "multiply_equal",
        BinOp::DivEq(_) => "divide_equal",
        BinOp::RemEq(_) => "remainder_equal",
        BinOp::BitXorEq(_) => "xor_equal",
        BinOp::BitAndEq(_) => "and_equal",
        BinOp::BitOrEq(_) => "or_equal",
        BinOp::ShlEq(_) => "shift_left_equal",
        BinOp::ShrEq(_) => "shift_right_equal",
    }
}
