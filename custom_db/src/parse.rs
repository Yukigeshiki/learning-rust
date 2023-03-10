use crate::ast::*;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser as _;

#[derive(Parser)]
#[grammar = "db.pest"]
pub struct Parser;

fn parse_tyrecord(mut pairs: Pairs<Rule>) -> Result<Ty, Error<Rule>> {
    let mut xs = Vec::new();

    while let Some(ident) = pairs.next() {
        let ty = parse_ty(pairs.next().unwrap().into_inner().next().unwrap())?;
        xs.push((ident.as_str().to_owned(), ty));
    }

    xs.sort_by(|(x, _), (y, _)| x.cmp(y));

    Ok(Ty::Record(xs))
}

fn parse_ty(pair: Pair<Rule>) -> Result<Ty, Error<Rule>> {
    match pair.as_rule() {
        Rule::tyident => match pair.as_str() {
            "Int" => Ok(Ty::Int),
            "Bool" => Ok(Ty::Bool),
            x => Err(Error::new_from_span(
                pest::error::ErrorVariant::CustomError {
                    message: format!("Invalid type {}", x),
                },
                pair.as_span(),
            )),
        },
        Rule::tytuple => Ok(Ty::Tuple(
            pair.into_inner()
                .map(|x| parse_ty(x.into_inner().next().unwrap()))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        Rule::unit => Ok(Ty::Unit),
        Rule::tyrecord => parse_tyrecord(pair.into_inner()),
        r => Err(Error::new_from_span(
            pest::error::ErrorVariant::CustomError {
                message: format!(
                    "Unexpected rule {:?}, expected tyindent, tyrecord, unit or tytuple",
                    r
                ),
            },
            pair.as_span(),
        )),
    }
}

fn parse_record(mut pairs: Pairs<Rule>) -> Result<Expr, Error<Rule>> {
    let mut xs = Vec::new();

    while let Some(ident) = pairs.next() {
        let expr = parse_expr(pairs.next().unwrap().into_inner().next().unwrap())?;
        xs.push((ident.as_str().to_owned(), expr));
    }

    xs.sort_by(|(x, _), (y, _)| x.cmp(y));

    Ok(Expr::Record(xs))
}

fn parse_expr(expr: Pair<Rule>) -> Result<Expr, Error<Rule>> {
    match expr.as_rule() {
        Rule::int => Ok(Expr::Int(expr.as_str().parse().unwrap())),
        Rule::bool => Ok(Expr::Bool(expr.as_str().parse().unwrap())),
        Rule::tuple => Ok(Expr::Tuple(
            expr.into_inner()
                .map(|x| parse_expr(x.into_inner().next().unwrap()))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        Rule::unit => Ok(Expr::Unit),
        Rule::record => parse_record(expr.into_inner()),
        r => Err(Error::new_from_span(
            pest::error::ErrorVariant::CustomError {
                message: format!("Unexpected rule {:?}, expected expr", r),
            },
            expr.as_span(),
        )),
    }
}

pub fn parse_select(mut pairs: Pairs<Rule>) -> Result<Statement, Error<Rule>> {
    let ident = pairs.next().unwrap().as_str();

    Ok(Statement::Select(ident.to_string()))
}

pub fn parse_insert(mut pairs: Pairs<Rule>) -> Result<Statement, Error<Rule>> {
    let expr = parse_expr(pairs.next().unwrap().into_inner().next().unwrap())?;
    let ident = pairs.next().unwrap().as_str();

    Ok(Statement::Insert(ident.to_string(), expr))
}

pub fn parse_create(mut pairs: Pairs<Rule>) -> Result<Statement, Error<Rule>> {
    let ident = pairs.next().unwrap().as_str();
    let ty = parse_ty(pairs.next().unwrap().into_inner().next().unwrap())?;

    Ok(Statement::Create(
        ident.to_string(),
        TableDefinition { ty: ty },
    ))
}

fn parse_statement(pair: Pair<Rule>) -> Result<Statement, Error<Rule>> {
    match pair.as_rule() {
        Rule::create => Ok(parse_create(pair.into_inner())?),
        Rule::select => Ok(parse_select(pair.into_inner())?),
        Rule::insert => Ok(parse_insert(pair.into_inner())?),
        _ => Err(Error::new_from_span(
            pest::error::ErrorVariant::CustomError {
                message: format!("Unexpected rule {:?}, expected statement", pair),
            },
            pair.as_span(),
        )),
    }
}

pub fn parse(input: &str) -> Result<Statement, Error<Rule>> {
    let statement = Parser::parse(Rule::statement, input)?.next().unwrap();

    parse_statement(statement)
}

#[cfg(test)]
mod test {
    use super::Ty;
    use super::*;

    #[test]
    fn parse_create_int() {
        assert_eq!(
            Statement::Create(String::from("x"), TableDefinition { ty: Ty::Int }),
            parse("create table x Int").unwrap()
        );
    }

    #[test]
    fn parse_create_bool() {
        assert_eq!(
            Statement::Create(String::from("x"), TableDefinition { ty: Ty::Bool }),
            parse("create table x Bool").unwrap()
        );
    }

    #[test]
    fn parse_create_tuple() {
        assert_eq!(
            Statement::Create(
                String::from("x"),
                TableDefinition {
                    ty: Ty::Tuple(vec!(Ty::Bool, Ty::Int))
                }
            ),
            parse("create table x (Bool, Int)").unwrap()
        );
    }

    #[test]
    fn parse_create_nested_tuple() {
        assert_eq!(
            Statement::Create(
                String::from("x"),
                TableDefinition {
                    ty: Ty::Tuple(vec!(Ty::Bool, Ty::Int, Ty::Tuple(vec!(Ty::Int, Ty::Int))))
                }
            ),
            parse("create table x (Bool, Int, (Int, Int,))").unwrap()
        );
    }

    #[test]
    fn parse_create_unit() {
        assert_eq!(
            Statement::Create(String::from("x"), TableDefinition { ty: Ty::Unit }),
            parse("create table x ()").unwrap()
        );
    }

    #[test]
    fn parse_create_record() {
        assert_eq!(
            Statement::Create(
                String::from("x"),
                TableDefinition {
                    ty: Ty::Record(vec!(
                        (String::from("x"), Ty::Bool),
                        (String::from("y"), Ty::Int)
                    ))
                }
            ),
            parse("create table x { y : Int, x : Bool }").unwrap()
        );
    }

    #[test]
    fn parse_insert_int() {
        assert_eq!(
            Statement::Insert(String::from("x"), Expr::Int(42)),
            parse("insert 42 into x").unwrap()
        )
    }

    #[test]
    fn parse_insert_negative_int() {
        assert_eq!(
            Statement::Insert(String::from("x"), Expr::Int(-42)),
            parse("insert -42 into x").unwrap()
        )
    }

    #[test]
    fn parse_insert_0() {
        assert_eq!(
            Statement::Insert(String::from("x"), Expr::Int(0)),
            parse("insert 0 into x").unwrap()
        )
    }

    #[test]
    fn parse_insert_negative_0() {
        assert_eq!(
            Statement::Insert(String::from("x"), Expr::Int(0)),
            parse("insert -0 into x").unwrap()
        )
    }

    #[test]
    fn parse_insert_bool() {
        assert_eq!(
            Statement::Insert(String::from("x"), Expr::Bool(false)),
            parse("insert false into x").unwrap()
        );
    }

    #[test]
    fn parse_insert_tuple() {
        assert_eq!(
            Statement::Insert(
                String::from("x"),
                Expr::Tuple(vec!(Expr::Bool(false), Expr::Bool(true), Expr::Int(42)))
            ),
            parse("insert (false, true, 42) into x").unwrap()
        );
    }

    #[test]
    fn parse_insert_unit() {
        assert_eq!(
            Statement::Insert(String::from("x"), Expr::Unit),
            parse("insert () into x").unwrap()
        );
    }

    #[test]
    fn parse_insert_record() {
        assert_eq!(
            Statement::Insert(
                String::from("x"),
                Expr::Record(vec!(
                    (String::from("x"), Expr::Bool(false)),
                    (String::from("y"), Expr::Int(42)),
                ))
            ),
            parse("insert { y = 42, x = false, } into x").unwrap()
        );
    }

    #[test]
    fn parse_insert_record_2() {
        assert_eq!(
            Statement::Insert(
                String::from("bar"),
                Expr::Record(vec!(
                    (String::from("x"), Expr::Int(0)),
                    (String::from("y"), Expr::Int(42)),
                ))
            ),
            parse("insert { y = 42, x = 0 } into bar").unwrap()
        );
    }

    #[test]
    fn parse_select() {
        assert_eq!(
            Statement::Select(String::from("x")),
            parse("select from x").unwrap()
        );
    }
}