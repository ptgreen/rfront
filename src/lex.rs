
use syntax::parse::token;
use syntax::parse::token::Token;
use syntax::parse::token::intern;

pub type Lexer<R> = RustLexer<R>;

rustlex! RustLexer {

    let DEC = ['0'-'9']+;

    "=" => |_| Some(token::Eq)
    "<" => |_| Some(token::Lt)
    "<=" => |_| Some(token::Le)
    "==" => |_| Some(token::EqEq)
    "!=" => |_| Some(token::Ne)
    ">=" => |_| Some(token::Ge)
    ">" => |_| Some(token::Gt)
    "&&" => |_| Some(token::AndAnd)
    "||" => |_| Some(token::OrOr)
    "!" => |_| Some(token::Not)
    "~" => |_| Some(token::Tilde)

    "+" => |_| Some(token::BinOp(token::Plus))
    "-" => |_| Some(token::BinOp(token::Minus))
    "*" => |_| Some(token::BinOp(token::Star))
    "/" => |_| Some(token::BinOp(token::Slash))
    "%" => |_| Some(token::BinOp(token::Percent))
    "^" => |_| Some(token::BinOp(token::Caret))
    "&" => |_| Some(token::BinOp(token::And))
    "|" => |_| Some(token::BinOp(token::Or))
    "<<" => |_| Some(token::BinOp(token::Shl))
    ">>" => |_| Some(token::BinOp(token::Shr))

    "+=" => |_| Some(token::BinOpEq(token::Plus))
    "-=" => |_| Some(token::BinOpEq(token::Minus))
    "*=" => |_| Some(token::BinOpEq(token::Star))
    "/=" => |_| Some(token::BinOpEq(token::Slash))
    "%=" => |_| Some(token::BinOpEq(token::Percent))
    "^=" => |_| Some(token::BinOpEq(token::Caret))
    "&=" => |_| Some(token::BinOpEq(token::And))
    "|=" => |_| Some(token::BinOpEq(token::Or))
    "<<=" => |_| Some(token::BinOpEq(token::Shl))
    ">>=" => |_| Some(token::BinOpEq(token::Shr))

    "@" => |_| Some(token::At)
    "." => |_| Some(token::Dot)
    ".." => |_| Some(token::DotDot)
    "..." => |_| Some(token::DotDotDot)
    "," => |_| Some(token::Comma)
    ";" => |_| Some(token::Semi)
    ":" => |_| Some(token::Colon)
    "::" => |_| Some(token::ModSep)
    "->" => |_| Some(token::RArrow)
    "<-" => |_| Some(token::LArrow)
    "=>" => |_| Some(token::FatArrow)
    "#" => |_| Some(token::Pound)
    "$" => |_| Some(token::Dollar)
    "?" => |_| Some(token::Question)

    "(" => |_| Some(token::OpenDelim(token::Paren))
    "[" => |_| Some(token::OpenDelim(token::Bracket))
    "{" => |_| Some(token::OpenDelim(token::Brace))
    ")" => |_| Some(token::CloseDelim(token::Paren))
    "]" => |_| Some(token::CloseDelim(token::Bracket))
    "}" => |_| Some(token::CloseDelim(token::Brace))

    DEC => |lex:&mut RustLexer<R>| {
        let name = intern(lex.yystr().as_slice());
        Some(token::Literal(token::Integer(name), None))
    }

    // "" => |_| Some(token::)
}

#[test]
fn test_lex() {
    use std::io::BufReader;
    let expected = vec!( token::Le, token::Not, token::AndAnd,
                         token::Literal(token::Integer(intern("7654")), None));
    let str = "<= ! && 7654";
    let inp = BufReader::new(str.as_bytes());
    let mut lexer = RustLexer::new(inp);
    let mut iter = expected.iter();
    for tok in *lexer {
        let expect = iter.next().unwrap();
        println!("{}", tok);
        assert!(expect == &tok);
    }
    assert!(iter.next() == None);
}
