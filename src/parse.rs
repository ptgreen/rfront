#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use racc::runtime::{ParserTables, ParserState};

use syntax::parse::token;
use syntax::codemap::{spanned, mk_sp, BytePos};
use syntax::ast;
use syntax::ptr::P;
use std;

#[deriving(Show)]
struct Context {
    x: ()
}

#[deriving(Show)]
enum Value {
    RawToken(token::Token),
    ASTItem(P<ast::Item>),
    ASTBlock(P<ast::Block>),
    ASTStmt(P<ast::Stmt>),
    ASTPat(P<ast::Pat>),
    ASTExpr(P<ast::Expr>),
    ASTTy(P<ast::Ty>)
}

pub type Parser = ParserState<Context, Option<Value>>;
const Z : BytePos = BytePos(0);

grammar! {

    Context cx;
    Option<Value>;

    LIT;
    EQEQ;

    Expr : LIT=lit {
        match lit {
            Some(Value::RawToken(token::Literal(token::Integer(name), None))) => {
                let n = std::num::from_str_radix(name.as_str(), 10).unwrap();
                let elit =
                    P(spanned(Z, Z,
                              ast::LitInt(n, ast::UnsuffixedIntLit(ast::Plus))));
                Some(Value::ASTExpr(P(ast::Expr { id: 0,
                                                  node: ast::ExprLit(elit),
                                                  span: mk_sp(Z, Z) })))
            }
            _ => panic!("unexpected lit value")
        }
    }
    | Expr=a EQEQ Expr=b {
        match (a, b) {
            (Some(Value::ASTExpr(aa)), Some(Value::ASTExpr(bb))) => {
                Some(Value::ASTExpr(P(ast::Expr { id: 0,
                                                  node: ast::ExprBinary(ast::BiEq, aa, bb),
                                                  span: mk_sp(Z, Z) })))
            }
            _ => panic!("unexpected binop '==' values")
        }
    }
    ;
}

#[test]
fn test_parse()
{
    use syntax::parse::token::intern;
    use racc::runtime::FinishParseResult;

    let toks = vec![
        (LIT, token::Literal(token::Integer(intern("7654")), None)),
        (EQEQ, token::EqEq),
        (LIT, token::Literal(token::Integer(intern("1234")), None))
    ];

    let mut parser = ParserState::new(get_parser_tables());
    let mut cx = Context { x: () };

    for &(tok, ref val) in toks.iter() {
        parser.push_token(&mut cx, tok, Some(Value::RawToken(val.clone())));
    }

    match parser.finish(&mut cx) {
        FinishParseResult::Accepted(final_value) => {
            println!("Accepted: {}", final_value);
        }
        FinishParseResult::SyntaxError => {
            panic!("SyntaxError");
        }
    }
}
