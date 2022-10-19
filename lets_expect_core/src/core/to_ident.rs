use english_numbers::{convert, Formatting};
use proc_macro2::Ident;
use syn::{
    punctuated::Punctuated, token::Comma, BinOp, Expr, ExprAssign, ExprRange, Lit, Local, Member,
    Pat, Stmt, Type, UnOp,
};

pub fn stmt_to_ident(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Local(local) => local_to_ident(local),
        Stmt::Item(_) => todo!("Item not supported"),
        Stmt::Expr(expr) => expr_to_ident(expr),
        Stmt::Semi(_, _) => todo!("Semi not supported"),
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
        _ => todo!("Unable to convert pattern to identifier"),
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
        Expr::Async(_) => todo!("Async not supported"),
        Expr::Await(_) => todo!("Await not supported"),
        Expr::Binary(binary) => {
            expr_to_ident(&binary.left)
                + "_"
                + binary_op_to_ident(&binary.op)
                + "_"
                + &expr_to_ident(&binary.right)
        }
        Expr::Block(block) => block_to_ident(block),
        Expr::Box(boxed) => "boxed_".to_string() + &expr_to_ident(&boxed.expr),
        Expr::Break(_) => todo!("Break not supported"),
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
        Expr::Closure(_) => todo!("Closure not supported"),
        Expr::Continue(_) => todo!("Continue not supported"),
        Expr::Field(field) => {
            expr_to_ident(&field.base) + "_" + member_to_ident(&field.member).as_str()
        }
        Expr::ForLoop(_) => todo!("ForLoop not supported"),
        Expr::Group(_) => todo!("Group not supported"),
        Expr::If(if_expr) => "if_".to_string() + &expr_to_ident(&if_expr.cond),
        Expr::Index(_) => todo!("Index not supported"),
        Expr::Let(_) => todo!("Let not supported"),
        Expr::Lit(lit) => expr_lit_to_ident(lit),
        Expr::Loop(_) => todo!("Loop not supported"),
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
        Expr::Repeat(_) => todo!("Repeat not supported"),
        Expr::Return(_) => todo!("Return not supported"),
        Expr::Struct(struc) => struc.path.get_ident().unwrap().to_string().to_lowercase(),
        Expr::Try(_) => todo!("Try not supported"),
        Expr::TryBlock(_) => todo!("TryBlock not supported"),
        Expr::Tuple(_) => todo!("Tuple not supported"),
        Expr::Type(_) => todo!("Type not supported"),
        Expr::Unary(unary) => {
            unary_op_to_ident(&unary.op).to_string() + "_" + &expr_to_ident(&unary.expr)
        }
        Expr::Unsafe(_) => todo!("Unsafe not supported"),
        Expr::Verbatim(_) => todo!("Verbatim not supported"),
        Expr::While(_) => todo!("While not supported"),
        Expr::Yield(_) => todo!("Yield not supported"),
        _ => todo!("Expected Expr, got {:?}", expr),
    }
}

fn type_to_ident(ty: &Type) -> String {
    match ty {
        Type::Array(array) => format!(
            "array_of_{}_{}",
            expr_to_ident(&array.len),
            type_to_ident(&array.elem)
        ),
        Type::BareFn(_) => todo!(),
        Type::Group(_) => todo!(),
        Type::ImplTrait(_) => todo!(),
        Type::Infer(_) => todo!(),
        Type::Macro(_) => todo!(),
        Type::Never(_) => todo!(),
        Type::Paren(_) => todo!(),
        Type::Path(path) => path
            .path
            .segments
            .last()
            .unwrap()
            .ident
            .to_string()
            .to_lowercase(),
        Type::Ptr(_) => todo!(),
        Type::Reference(_) => todo!(),
        Type::Slice(_) => todo!(),
        Type::TraitObject(_) => todo!(),
        Type::Tuple(_) => todo!(),
        Type::Verbatim(_) => todo!(),
        _ => todo!(),
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
    let from = range.from.as_ref().map(|expr| expr_to_ident(expr)).unwrap();
    let to = range.to.as_ref().map(|expr| expr_to_ident(expr)).unwrap();
    format!("range_from_{}_to_{}", from, to)
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
        Lit::ByteStr(_) => todo!(),
        Lit::Byte(_) => todo!(),
        Lit::Char(_) => todo!(),
        Lit::Int(value) => convert(
            value.base10_parse::<i64>().unwrap(),
            Formatting {
                dashes: false,
                title_case: false,
                ..Formatting::default()
            },
        ),
        Lit::Float(_) => todo!(),
        Lit::Bool(value) => value.value.to_string(),
        Lit::Verbatim(_) => todo!(),
    }
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
        BinOp::BitXor(_) => "xor",
        BinOp::BitAnd(_) => "and",
        BinOp::BitOr(_) => "or",
        BinOp::Shl(_) => "shift_left_by",
        BinOp::Shr(_) => "shift_right_by",
        BinOp::Eq(_) => "equals",
        BinOp::Lt(_) => "less_than",
        BinOp::Le(_) => "less_equal_than",
        BinOp::Ne(_) => "not_equal_to",
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
