#![allow(dead_code)]
#![allow(unused_variables)]
use quote::quote;
use syn::{parse_macro_input, ItemFn, Stmt, Expr};


#[proc_macro_attribute]
pub fn another_one(attr: proc_macro::TokenStream, items: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item_clone = items.clone();
    let mut func = parse_macro_input!(item_clone as ItemFn);
    let mut new_block = Vec::new();
    new_block.push(syn::parse_str("x = x + 1;").unwrap());
    new_block.append(&mut (*func.block).stmts.clone());
    func.block.stmts = new_block;
    let generated = quote!{
        #func
    }.into();
    println!("another_one generation: {}", generated);
    generated
}

#[proc_macro_attribute]
pub fn do_10_times(attr: proc_macro::TokenStream, items: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut func = parse_macro_input!(items as ItemFn);
    let return_line = (*func.block).stmts.remove((*func.block).stmts.len() - 1);
    let new_loop = syn::ExprForLoop{
        attrs: Vec::new(),
        label: None,
        for_token: syn::token::For::default(),
        pat: syn::parse_str("_").unwrap(),
        in_token: syn::token::In::default(),
        expr: syn::parse_str("0..10").unwrap(),
        body: (*func.block).clone()
    };
    let mut new_block = vec![Stmt::Expr(Expr::ForLoop(new_loop))];
    new_block.push(return_line);
    func.block.stmts = new_block;
    let generated = quote!{
        #func
    }.into();
    //println!("do_10_tines generation: {}", generated);
    generated
}