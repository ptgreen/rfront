#![feature(phase)]
#![allow(dead_code)]

/*
 * The goal is to construct existing rustc-frontend types from
 * libsyntax, but using a rustlex lexer and a racc parser. So we link
 * to libsyntax here in order to get the AST and token types.
 */
extern crate syntax;

#[phase(plugin,link)] extern crate log;
#[phase(plugin,link)] extern crate rustlex;
#[phase(plugin, link)] extern crate racc;

mod lex;
mod parse;

struct Session<'a>
{
    lexer: lex::Lexer<std::io::BufReader<'a>>,
    parser: parse::Parser
}
