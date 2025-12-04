use crate::ast::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace1},
    combinator::{map, map_res, opt, recognize, value, verify},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

fn comment<'a, E: nom::error::ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value(
        (),
        verify(
            tuple((tag("//"), take_while(|c| c != '\n'), char('\n'))),
            |(_, s, _): &(&str, &str, char)| {
                if s.starts_with('!') {
                    return false;
                }
                if s.starts_with('/') {
                    // If it starts with /, it's at least ///.
                    // We accept //// (s starts with //) as ignored comment.
                    // We reject /// (s starts with / but not //) as doc comment.
                    return s.starts_with("//");
                }
                true
            },
        ),
    )(input)
}

fn doc_comment<'a, E: nom::error::ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, String, E> {
    map(
        verify(
            tuple((tag("///"), take_while(|c| c != '\n'), char('\n'))),
            |(_, s, _): &(&str, &str, char)| !s.starts_with('/'), // Reject ////
        ),
        |(_, s, _)| s.trim().to_string(),
    )(input)
}

fn mod_comment<'a, E: nom::error::ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, String, E> {
    map(
        tuple((tag("//!"), take_while(|c: char| c != '\n'), char('\n'))),
        |(_, s, _): (&str, &str, char)| s.trim().to_string(),
    )(input)
}

fn ws<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    preceded(
        many0(alt((value((), multispace1), comment))),
        inner,
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
    expr_or(input)
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

fn expr_unary(input: &str) -> IResult<&str, Expr> {
    alt((
        expr_atom,
        map(pair(ws(tag("!")), ws(expr_unary)), |(_, val)| {
            Expr::UnOp(UnOp::Not, Box::new(val))
        }),
        map(pair(ws(char('-')), ws(expr_unary)), |(_, val)| {
            Expr::UnOp(UnOp::Neg, Box::new(val))
        }),
    ))(input)
}

fn expr_mul_div(input: &str) -> IResult<&str, Expr> {
    let (input, init) = ws(expr_unary)(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((char('*'), char('/'), char('%')))),
        ws(expr_unary),
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

fn expr_or(input: &str) -> IResult<&str, Expr> {
    let (input, init) = ws(expr_and)(input)?;
    let (input, rest) = many0(tuple((ws(map(tag("||"), |_| Op::Or)), ws(expr_and))))(input)?;

    Ok((
        input,
        rest.into_iter().fold(init, |acc, (op, val)| {
            Expr::BinOp(Box::new(acc), op, Box::new(val))
        }),
    ))
}

fn expr_and(input: &str) -> IResult<&str, Expr> {
    let (input, init) = ws(expr_eq)(input)?;
    let (input, rest) = many0(tuple((ws(map(tag("&&"), |_| Op::And)), ws(expr_eq))))(input)?;

    Ok((
        input,
        rest.into_iter().fold(init, |acc, (op, val)| {
            Expr::BinOp(Box::new(acc), op, Box::new(val))
        }),
    ))
}

fn expr_eq(input: &str) -> IResult<&str, Expr> {
    let (input, init) = ws(expr_cmp)(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((
            map(tag("=="), |_| Op::Eq),
            map(tag("!="), |_| Op::Ne),
            map(char('='), |_| Op::Eq),
        ))),
        ws(expr_cmp),
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(init, |acc, (op, val)| {
            Expr::BinOp(Box::new(acc), op, Box::new(val))
        }),
    ))
}

fn expr_cmp(input: &str) -> IResult<&str, Expr> {
    let (input, init) = ws(expr_sum)(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((
            map(tag(">="), |_| Op::Ge),
            map(tag("<="), |_| Op::Le),
            map(char('>'), |_| Op::Gt),
            map(char('<'), |_| Op::Lt),
        ))),
        ws(expr_sum),
    )))(input)?;

    Ok((
        input,
        rest.into_iter().fold(init, |acc, (op, val)| {
            Expr::BinOp(Box::new(acc), op, Box::new(val))
        }),
    ))
}

fn expr_sum(input: &str) -> IResult<&str, Expr> {
    let (input, init) = ws(expr_mul_div)(input)?;
    let (input, rest) = many0(tuple((
        ws(alt((
            map(char('+'), |_| Op::Add),
            map(char('-'), |_| Op::Sub),
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
fn attach_comment(stmt: Stmt, comment: String) -> Stmt {
    match stmt {
        Stmt::VarDecl(n, e, _) => Stmt::VarDecl(n, e, Some(comment)),
        Stmt::Assign(n, e, _) => Stmt::Assign(n, e, Some(comment)),
        Stmt::Expr(e, _) => Stmt::Expr(e, Some(comment)),
        Stmt::If(c, t, e, _) => Stmt::If(c, t, e, Some(comment)),
        Stmt::Repeat(c, b, _) => Stmt::Repeat(c, b, Some(comment)),
        Stmt::Forever(b, _) => Stmt::Forever(b, Some(comment)),
        Stmt::Until(c, b, _) => Stmt::Until(c, b, Some(comment)),
        Stmt::Match(e, c, d, _) => Stmt::Match(e, c, d, Some(comment)),
        Stmt::Return(e, _) => Stmt::Return(e, Some(comment)),
        Stmt::CBlock(n, a, b, _) => Stmt::CBlock(n, a, b, Some(comment)),
        Stmt::Comment(_) => stmt,
    }
}

fn stmt(input: &str) -> IResult<&str, Stmt> {
    let (input, comment) = opt(ws(doc_comment))(input)?;
    let (input, mut s) = alt((
        map(ws(mod_comment), Stmt::Comment),
        stmt_if,
        stmt_match,
        stmt_repeat,
        stmt_forever,
        stmt_until,
        stmt_var_decl,
        stmt_assign,
        stmt_return,
        stmt_c_block,
        stmt_expr,
    ))(input)?;

    if let Some(c) = comment {
        s = attach_comment(s, c);
    }
    Ok((input, s))
}

fn stmt_c_block(input: &str) -> IResult<&str, Stmt> {
    let (input, (name, args)) = ws(func_call)(input)?;
    let (input, body) = ws(block)(input)?;
    Ok((input, Stmt::CBlock(name, args, body, None)))
}

fn block(input: &str) -> IResult<&str, Vec<Stmt>> {
    delimited(ws(char('{')), many0(ws(stmt)), ws(char('}')))(input)
}

fn stmt_match(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = ws(tag("match"))(input)?;
    let (input, expr) = ws(expr)(input)?;
    let (input, cases) =
        delimited(ws(char('{')), many0(ws(stmt_match_case)), ws(char('}')))(input)?;

    let mut match_cases = Vec::new();
    let mut default_case = None;

    for (case_expr, body) in cases {
        if let Some(e) = case_expr {
            match_cases.push((e, body));
        } else {
            if default_case.is_some() {
                // In a real compiler we should report error, but nom makes it hard.
                // We'll just ignore subsequent default cases or take the last one.
            }
            default_case = Some(body);
        }
    }

    Ok((input, Stmt::Match(expr, match_cases, default_case, None)))
}

fn stmt_match_case(input: &str) -> IResult<&str, (Option<Expr>, Vec<Stmt>)> {
    let (input, expr) = ws(alt((map(tag("_"), |_| None), map(expr, Some))))(input)?;
    let (input, _) = ws(tag("=>"))(input)?;
    let (input, body) = ws(block)(input)?;
    let (input, _) = opt(ws(char(',')))(input)?; // Optional trailing comma
    Ok((input, (expr, body)))
}

fn stmt_if(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = ws(tag("if"))(input)?;
    let (input, cond) = ws(expr)(input)?;
    let (input, then_block) = ws(block)(input)?;
    let (input, else_block) = opt(preceded(ws(tag("else")), ws(block)))(input)?;

    Ok((input, Stmt::If(cond, then_block, else_block, None)))
}

fn stmt_repeat(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = ws(tag("repeat"))(input)?;
    let (input, count) = delimited(ws(char('(')), ws(expr), ws(char(')')))(input)?;
    let (input, body) = ws(block)(input)?;
    Ok((input, Stmt::Repeat(count, body, None)))
}

fn stmt_forever(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = ws(tag("forever"))(input)?;
    let (input, body) = ws(block)(input)?;
    Ok((input, Stmt::Forever(body, None)))
}

fn stmt_until(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = ws(tag("until"))(input)?;
    let (input, cond) = ws(expr)(input)?;
    let (input, body) = ws(block)(input)?;
    Ok((input, Stmt::Until(cond, body, None)))
}

fn stmt_var_decl(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = ws(tag("let"))(input)?;
    let (input, name) = ws(identifier)(input)?;
    let (input, _) = ws(char('='))(input)?;
    let (input, val) = ws(expr)(input)?;
    let (input, _) = ws(char(';'))(input)?;
    Ok((input, Stmt::VarDecl(name, val, None)))
}

fn stmt_assign(input: &str) -> IResult<&str, Stmt> {
    let (input, name) = ws(identifier)(input)?;
    let (input, op) = ws(alt((tag("="), tag("+="), tag("-="))))(input)?;
    let (input, val) = ws(expr)(input)?;
    let (input, _) = ws(char(';'))(input)?;

    match op {
        "=" => Ok((input, Stmt::Assign(name, val, None))),
        "+=" => Ok((
            input,
            Stmt::Assign(
                name.clone(),
                Expr::BinOp(Box::new(Expr::Variable(name)), Op::Add, Box::new(val)),
                None,
            ),
        )),
        "-=" => Ok((
            input,
            Stmt::Assign(
                name.clone(),
                Expr::BinOp(Box::new(Expr::Variable(name)), Op::Sub, Box::new(val)),
                None,
            ),
        )),
        _ => unreachable!(),
    }
}

fn stmt_return(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = ws(tag("return"))(input)?;
    let (input, val) = opt(ws(expr))(input)?;
    let (input, _) = ws(char(';'))(input)?;
    Ok((input, Stmt::Return(val, None)))
}

fn stmt_expr(input: &str) -> IResult<&str, Stmt> {
    let (input, e) = ws(expr)(input)?;
    let (input, _) = ws(char(';'))(input)?;
    Ok((input, Stmt::Expr(e, None)))
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

fn item_mod_comment(input: &str) -> IResult<&str, Item> {
    map(ws(mod_comment), Item::Comment)(input)
}

fn item_var_decl(input: &str) -> IResult<&str, Item> {
    let (input, comment) = opt(ws(doc_comment))(input)?;
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
            comment,
        }),
    ))
}

fn item_procedure(input: &str) -> IResult<&str, Item> {
    let (input, comment) = opt(ws(doc_comment))(input)?;
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

    let is_warp = attributes.iter().any(|a| a.name == "warp")
        && !attributes.iter().any(|a| a.name == "nowarp");

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
            comment,
        }),
    ))
}

fn item_costume(input: &str) -> IResult<&str, Item> {
    let (input, _comment) = opt(ws(doc_comment))(input)?;
    let (input, _) = ws(tag("costume"))(input)?;
    let (input, name) = ws(string_literal)(input)?;
    let (input, path) = ws(string_literal)(input)?;
    let (input, coords) = opt(pair(ws(number_literal), ws(number_literal)))(input)?;
    let (input, _) = ws(char(';'))(input)?;

    let (x, y) = match coords {
        Some((cx, cy)) => (Some(cx), Some(cy)),
        None => (None, None),
    };

    Ok((input, Item::Costume(AssetDecl { name, path, x, y })))
}

fn item_sound(input: &str) -> IResult<&str, Item> {
    let (input, _comment) = opt(ws(doc_comment))(input)?;
    let (input, _) = ws(tag("sound"))(input)?;
    let (input, name) = ws(string_literal)(input)?;
    let (input, path) = ws(string_literal)(input)?;
    let (input, _) = ws(char(';'))(input)?;
    Ok((
        input,
        Item::Sound(AssetDecl {
            name,
            path,
            x: None,
            y: None,
        }),
    ))
}

fn item_function(input: &str) -> IResult<&str, Item> {
    let (input, comment) = opt(ws(doc_comment))(input)?;
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

    let is_warp = attributes.iter().any(|a| a.name == "warp")
        && !attributes.iter().any(|a| a.name == "nowarp");

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
            comment,
        }),
    ))
}

fn item_stmt(input: &str) -> IResult<&str, Item> {
    map(stmt, Item::Stmt)(input)
}

fn item(input: &str) -> IResult<&str, Item> {
    ws(alt((
        item_mod_comment,
        item_var_decl,
        item_costume,
        item_sound,
        item_function,
        item_procedure,
        item_stmt,
    )))(input)
}

enum BreakItem {
    Newlines(usize),
    Comment,
}

pub fn parse_program(input: &str) -> IResult<&str, Program> {
    let mut items = Vec::new();
    let mut input = input;

    loop {
        let mut newline_count = 0;
        // Consume whitespace and check for blank lines
        let (next_input, tokens) = many0(alt((
            map(multispace1, |s: &str| {
                BreakItem::Newlines(s.chars().filter(|c| *c == '\n').count())
            }),
            map(comment, |_| BreakItem::Comment),
        )))(input)?;
        
        for token in tokens {
            match token {
                BreakItem::Newlines(n) => newline_count += n,
                BreakItem::Comment => newline_count += 1,
            }
        }

        input = next_input;

        if newline_count >= 2 {
            items.push(Item::BatchBreak);
        }

        if input.is_empty() {
            break;
        }

        match item(input) {
            Ok((next_input, it)) => {
                items.push(it);
                input = next_input;
            }
            Err(e) => return Err(e),
        }
    }

    Ok((input, Program { items }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func_comment() {
        let input = "
        /// My comment
        #[on_flag_clicked]
        fn start() {
        }
        ";
        let res = item_function(input);
        if let Err(e) = &res {
            println!("Error: {:?}", e);
        }
        assert!(res.is_ok());
        let (_, item) = res.unwrap();
        if let Item::Function(f) = item {
            assert_eq!(f.comment, Some("My comment".to_string()));
            assert_eq!(f.attributes.len(), 1);
        } else {
            panic!("Not a function");
        }
    }

    #[test]
    fn test_stmt_comment() {
        let input = "
        /// My stmt comment
        move(10);
        ";
        let res = stmt(input);
        assert!(res.is_ok());
        let (_, s) = res.unwrap();
        match s {
            Stmt::Expr(_, comment) => assert_eq!(comment, Some("My stmt comment".to_string())),
            _ => panic!("Expected Expr stmt"),
        }
    }
}
