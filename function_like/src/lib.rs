#![allow(dead_code)]
#![allow(unused_variables)]
extern crate proc_macro;
use syn::{parse_macro_input, token::Brace};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::Stmt;
use std::boxed::Box;

use quote::{quote, ToTokens};
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    Block,
    Expr,
    ExprWhile,
    token::{Bracket, While},
    Token,
};
use std::vec::Vec;

#[derive(Debug)]
struct BFCode {
    code: Vec<BF>
}

#[derive(Debug)]
enum BF{
    Op(Operation),
    Block(Box<BFCode>)
}

#[derive(Debug)]
enum Operation{
    IncrementPtr,
    DecrementPtr,
    IncrementVal,
    DecrementVal,
    Output,
    Accept
}

#[derive(Debug)]
enum BFBlockModel {
    Stmt(Stmt),
    While(ExprWhile)
}
#[derive(Debug)]
struct BFModel {
    stmts: Vec<Stmt> 
}

fn read_from_stdin() -> u8 {
    let mut input_line = String::new();
    std::io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let x: u8 = input_line.trim().parse().expect("Input not an integer");
    x
}
fn op_to_stmt(op: Operation) -> Stmt {
    match op{
        Operation::Accept =>{
            syn::parse_str("tape[ptr] = read_from_stdin();").unwrap()
        },
        Operation::Output =>{
            syn::parse_str("print!(\"{}\",tape[ptr] as char);").unwrap()
        },
        Operation::DecrementPtr =>{
            syn::parse_str("ptr = ptr - 1;").unwrap()
        },
        Operation::DecrementVal =>{
            syn::parse_str("tape[ptr] = tape[ptr] - 1;").unwrap()
        },
        Operation::IncrementPtr =>{
            syn::parse_str("ptr = ptr + 1;").unwrap()
        },
        Operation::IncrementVal =>{
            syn::parse_str("tape[ptr] = tape[ptr] + 1;").unwrap()
        },
    }
}
fn create_bd_stmts(code: BFCode) -> Block{
    let mut stmts = Vec::new();
    for ops in code.code{
        match ops{
            BF::Op(bf_operation) => {
                stmts.push(op_to_stmt(bf_operation));
            },
            BF::Block(bf_loop) => {
                let bf_loop = ExprWhile{
                    attrs: Vec::new(),
                    label: None,
                    while_token: While::default(),
                    cond: Box::new(syn::parse_str("tape[ptr] != 0").unwrap()),
                    body: create_bd_stmts(*bf_loop)
                };
                stmts.push(Stmt::Expr(Expr::While(bf_loop)));
            }
        }
    }
    Block{
        stmts,
        brace_token: Brace::default()
    }
}

fn create_bf_model(code: BFCode) -> BFModel {
    let mut new_code = Vec::new();
    let init_ptr: Stmt = syn::parse_str("let mut ptr = 0;").unwrap();
    let init_tape: Stmt = syn::parse_str("let mut tape:Vec<u8> = vec![0;30000];").unwrap();
    let mut bf_body = create_bd_stmts(code);
    new_code.push(init_ptr);
    new_code.push(init_tape);
    new_code.append(&mut bf_body.stmts);

    BFModel {
        stmts: new_code
    }
}

impl Parse for BFCode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut code = Vec::new();
        while !input.is_empty(){
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![>]) {
                input.parse::<Token![>]>()?;
                code.push(BF::Op(Operation::IncrementPtr));
            } else if lookahead.peek(Token![<]) {
                input.parse::<Token![<]>()?;
                code.push(BF::Op(Operation::DecrementPtr));
            } else if lookahead.peek(Token![+]) {
                input.parse::<Token![+]>()?;
                code.push(BF::Op(Operation::IncrementVal));
            } else if lookahead.peek(Token![-]) {
                input.parse::<Token![-]>()?;
                code.push(BF::Op(Operation::DecrementVal));
            } else if lookahead.peek(Token![.]) {
                input.parse::<Token![.]>()?;
                code.push(BF::Op(Operation::Output));
            } else if lookahead.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                code.push(BF::Op(Operation::Accept));
            } else if lookahead.peek(Bracket) {
                let content;
                bracketed!(content in input);
                let inner_content = content.parse::<BFCode>()?;
                code.push(BF::Block(Box::new(inner_content)));
            } else {
                return Err(lookahead.error())
            }
        }
        Ok(
            BFCode { code }
        )
    }
}
impl ToTokens for BFModel {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        for stmt in self.stmts.clone(){
            stmt.to_tokens(tokens);
        }
    }
}
#[proc_macro]
pub fn bf(item: TokenStream) -> TokenStream {
    let item_clone = item.clone();
    let representation = parse_macro_input!(item_clone as BFCode);
    let model = create_bf_model(representation);
    let generated: TokenStream = quote! {
        #model
    }.into();
    //println!("{}", generated.clone());
    generated
}