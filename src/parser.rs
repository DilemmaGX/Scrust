use crate::ast::*;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace1},
    combinator::{map, map_res, opt, recognize, value, verify},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, tuple},
};

fn comment<'a, E: nom::error::ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value(
        (),
        verify(
            tuple((tag("//"), take_while(|c| c != '\n'), char('\n'))),
            |(_, s, _): &(&str, &str, char)| !s.starts_with('/'),
        ),
    )(input)
}

fn ws<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(
        many0(alt((value((), multispace1), comment))),
        inner,
        many0(alt((value((), multispace1), comment))),
    )
}

fn identifier(input: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s: &str| s.to_string(),
    )(input)
}

// Types
fn type_spec(input: &str) -> IResult<&str, Type> {
    alt((
        value(Type::Number, tag("number")),
        value(Type::String, tag("string")),
        value(Type::Boolean, tag("boolean")),
        value(Type::List, tag("list")),
    ))(input)
}

// Literals
fn number_literal(input: &str) -> IResult<&str, f64> {
    map_res(
        recognize(tuple((
            opt(char('-')),
            digit1,
            opt(tuple((char('.'), digit1))),
        ))),
        |s: &str| s.parse::<f64>(),
    )(input)
}

fn string_literal(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        map(take_while(|c| c != '"'), |s: &str| s.to_string()),
        char('"'),
    )(input)
}

fn bool_literal(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("true")), value(false, tag("false"))))(input)
}

fn list_literal(input: &str) -> IResult<&str, Vec<Expr>> {
    delimited(
        ws(char('[')),
        separated_list0(ws(char(',')), ws(expr)),
        ws(char(']')),
    )(input)
}

fn expr(input: &str) -> IResult<&str, Expr> {
    expr_add_sub(input)
}

fn expr_atom(input: &str) -> IResult<&str, Expr> {
    alt((
        map(number_literal, Expr::Number),
        map(string_literal, Expr::String),
        map(bool_literal, Expr::Bool),
        map(list_literal, Expr::List),
        map(func_call, |(name, args)| Expr::Call(name, args)),
        map(identifier, Expr::Variable),
        delimited(char('('), expr, char(')')),
    ))(input)
}

fn expr_mul_div(input: &str) -> IResult<&str, Expr> {
    let (input, init) = ws(expr_atom)(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((char('*'), char('/'), char('%')))),
        ws(expr_atom),
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(init, |acc, (op, val)| {
            let op = match op {
                '*' => Op::Mul,
                '/' => Op::Div,
                '%' => Op::Mod,
                _ => unreachable!(),
            };
            Expr::BinOp(Box::new(acc), op, Box::new(val))
        }),
    ))
}

fn expr_add_sub(input: &str) -> IResult<&str, Expr> {
    let (input, init) = ws(expr_mul_div)(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((
            map(tag("=="), |_| Op::Eq),
            map(tag("!="), |_| Op::Ne),
            map(tag(">="), |_| Op::Ge),
            map(tag("<="), |_| Op::Le),
            map(tag("&&"), |_| Op::And),
            map(tag("||"), |_| Op::Or),
            map(char('+'), |_| Op::Add),
            map(char('-'), |_| Op::Sub),
            map(char('='), |_| Op::Eq),
            map(char('>'), |_| Op::Gt),
            map(char('<'), |_| Op::Lt),
        ))),
        ws(expr_mul_div),
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(init, |acc, (op, val)| {
            Expr::BinOp(Box::new(acc), op, Box::new(val))
        }),
    ))
}

fn func_call(input: &str) -> IResult<&str, (String, Vec<Expr>)> {
    pair(
        identifier,
        delimited(
            char('('),
            separated_list0(ws(char(',')), ws(expr)),
            char(')'),
        ),
    )(input)
}

// Statements
fn stmt(input: &str) -> IResult<&str, Stmt> {
    alt((
        stmt_if,
        stmt_repeat,
        stmt_forever,
        stmt_until,
        stmt_var_decl,
        stmt_assign,
        stmt_expr,
    ))(input)
}

fn block(input: &str) -> IResult<&str, Vec<Stmt>> {
    delimited(ws(char('{')), many0(ws(stmt)), ws(char('}')))(input)
}

fn stmt_if(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = ws(tag("if"))(input)?;
    let (input, cond) = ws(expr)(input)?;
    let (input, then_block) = ws(block)(input)?;
    let (input, else_block) = opt(preceded(ws(tag("else")), ws(block)))(input)?;

    Ok((input, Stmt::If(cond, then_block, else_block)))
}

fn stmt_repeat(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = ws(tag("repeat"))(input)?;
    let (input, count) = delimited(ws(char('(')), ws(expr), ws(char(')')))(input)?;
    let (input, body) = ws(block)(input)?;
    Ok((input, Stmt::Repeat(count, body)))
}

fn stmt_forever(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = ws(tag("forever"))(input)?;
    let (input, body) = ws(block)(input)?;
    Ok((input, Stmt::Forever(body)))
}

fn stmt_until(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = ws(tag("until"))(input)?;
    let (input, cond) = ws(expr)(input)?;
    let (input, body) = ws(block)(input)?;
    Ok((input, Stmt::Until(cond, body)))
}

fn stmt_var_decl(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = ws(tag("let"))(input)?;
    let (input, name) = ws(identifier)(input)?;
    let (input, _) = ws(char('='))(input)?;
    let (input, val) = ws(expr)(input)?;
    let (input, _) = ws(char(';'))(input)?;
    Ok((input, Stmt::VarDecl(name, val)))
}

fn stmt_assign(input: &str) -> IResult<&str, Stmt> {
    let (input, name) = ws(identifier)(input)?;
    let (input, op) = ws(alt((tag("="), tag("+="), tag("-="))))(input)?;
    let (input, val) = ws(expr)(input)?;
    let (input, _) = ws(char(';'))(input)?;

    match op {
        "=" => Ok((input, Stmt::Assign(name, val))),
        "+=" => Ok((
            input,
            Stmt::Assign(
                name.clone(),
                Expr::BinOp(Box::new(Expr::Variable(name)), Op::Add, Box::new(val)),
            ),
        )),
        "-=" => Ok((
            input,
            Stmt::Assign(
                name.clone(),
                Expr::BinOp(Box::new(Expr::Variable(name)), Op::Sub, Box::new(val)),
            ),
        )),
        _ => unreachable!(),
    }
}

fn stmt_expr(input: &str) -> IResult<&str, Stmt> {
    let (input, e) = ws(expr)(input)?;
    let (input, _) = ws(char(';'))(input)?;
    Ok((input, Stmt::Expr(e)))
}

fn attribute(input: &str) -> IResult<&str, Attribute> {
    map(
        delimited(
            ws(tag("#[")),
            pair(
                identifier,
                opt(delimited(
                    ws(char('(')),
                    separated_list0(ws(char(',')), ws(expr)),
                    ws(char(')')),
                )),
            ),
            ws(tag("]")),
        ),
        |(name, args)| Attribute {
            name,
            args: args.unwrap_or_default(),
        },
    )(input)
}

fn item_comment(input: &str) -> IResult<&str, Item> {
    map(
        tuple((ws(tag("///")), take_while(|c| c != '\n'), char('\n'))),
        |(_, content, _)| Item::Comment(content.trim().to_string()),
    )(input)
}

fn item_var_decl(input: &str) -> IResult<&str, Item> {
    let (input, vis) = opt(ws(alt((
        value(Visibility::Public, tag("public")),
        value(Visibility::Private, tag("private")),
    ))))(input)?;

    // Handle "var" and "list" keywords
    let (input, decl_type) = ws(alt((
        value(Type::Unknown, tag("var")),
        value(Type::List, tag("list")),
    )))(input)?;

    let (input, name) = ws(identifier)(input)?;
    let (input, _) = ws(char('='))(input)?;
    let (input, init) = ws(expr)(input)?;
    let (input, _) = ws(char(';'))(input)?;

    Ok((
        input,
        Item::Variable(VariableDecl {
            name,
            ty: decl_type,
            init,
            visibility: vis.unwrap_or(Visibility::Default),
        }),
    ))
}

fn item_procedure(input: &str) -> IResult<&str, Item> {
    let (input, attributes) = many0(ws(attribute))(input)?;
    let (input, _) = ws(tag("proc"))(input)?;
    let (input, name) = ws(identifier)(input)?;
    let (input, params) = delimited(
        ws(char('(')),
        separated_list0(
            ws(char(',')),
            pair(ws(identifier), preceded(ws(char(':')), ws(type_spec))),
        ),
        ws(char(')')),
    )(input)?;
    let (input, body) = ws(block)(input)?;

    let is_warp = attributes.iter().any(|a| a.name == "warp") && !attributes.iter().any(|a| a.name == "nowarp");

    Ok((
        input,
        Item::Procedure(ProcedureDef {
            name,
            params: params
                .into_iter()
                .map(|(n, t)| Param { name: n, ty: t })
                .collect(),
            body,
            is_warp,
        }),
    ))
}

fn item_costume(input: &str) -> IResult<&str, Item> {
    let (input, _) = ws(tag("costume"))(input)?;
    let (input, name) = ws(string_literal)(input)?;
    let (input, path) = ws(string_literal)(input)?;
    let (input, _) = ws(char(';'))(input)?;
    Ok((input, Item::Costume(AssetDecl { name, path })))
}

fn item_sound(input: &str) -> IResult<&str, Item> {
    let (input, _) = ws(tag("sound"))(input)?;
    let (input, name) = ws(string_literal)(input)?;
    let (input, path) = ws(string_literal)(input)?;
    let (input, _) = ws(char(';'))(input)?;
    Ok((input, Item::Sound(AssetDecl { name, path })))
}

fn item_function(input: &str) -> IResult<&str, Item> {
    let (input, attributes) = many0(ws(attribute))(input)?;

    let (input, _) = ws(tag("fn"))(input)?;
    let (input, name) = ws(identifier)(input)?;
    let (input, params) = delimited(
        ws(char('(')),
        separated_list0(
            ws(char(',')),
            pair(ws(identifier), preceded(ws(char(':')), ws(type_spec))),
        ),
        ws(char(')')),
    )(input)?;
    let (input, body) = ws(block)(input)?;

    let is_warp = attributes.iter().any(|a| a.name == "warp") && !attributes.iter().any(|a| a.name == "nowarp");

    Ok((
        input,
        Item::Function(Function {
            name,
            attributes,
            params: params
                .into_iter()
                .map(|(n, t)| Param { name: n, ty: t })
                .collect(),
            body,
            is_warp,
        }),
    ))
}

fn item_stmt(input: &str) -> IResult<&str, Item> {
    map(stmt, Item::Stmt)(input)
}

fn item(input: &str) -> IResult<&str, Item> {
    ws(alt((
        item_comment,
        item_var_decl,
        item_costume,
        item_sound,
        item_function,
        item_procedure,
        item_stmt,
    )))(input)
}

pub fn parse_program(input: &str) -> IResult<&str, Program> {
    map(many0(item), |items| Program { items })(input)
}
