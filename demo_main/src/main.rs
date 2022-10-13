#![allow(dead_code)]
#![allow(unused_variables)]
use declarative_macros::{set_things, call_trait, create_fn, bit_more_complicated, print_vars};
use function_like_macros::bf;
use attribute_macros::{another_one, do_n_times};
use derive_macros::Getters;

#[derive(Getters)]
struct FooBar2{
    boo: u32,
    far: u32,
}

#[derive(Getters)]
struct FooBar{
    foo: u32,
    bar: u32,
    bizz: u32
}

impl FooBar{
    fn a_number(self) -> u32{
        100
    }
}
fn demo_declarative(){

    let hello;
    let goodbye ;
    let evening ;

    set_things!(hello, 5; goodbye,42; evening,7);

    println!("Hello: {:?}", hello);
    println!("Goodbye: {:?}", goodbye);
    println!("Evening: {:?}", evening);
    //================================

    let mut x = 0;
    set_things!(x = x + 1, x = x + 7 , x = x - 5);
    println!("{:?}", x);

    //================================
    create_fn!();
    xfunc();

    //================================
    let a: &[i32] = bit_more_complicated!(10; [1, 2, 3];
           20; [4, 5, 6]);

    assert_eq!(a, [11, 12, 13, 24, 25, 26]);

    //================================
    let test_foobar = FooBar {
        foo: 5,
        bar: 6,
        bizz: 7
    };
    println!("{}", call_trait!(test_foobar));

    //================================
    print_vars!(y => 7);
}

fn demo_function_like(){
    bf!(++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.);
}

#[another_one]
#[do_n_times]
fn add_one(mut x: u8) -> u8{
    x = x + 1;
    x
}

fn demo_attribute(){
    println!("{}", add_one(5));
}

fn demo_derive(){
   let test_foobar = FooBar {
    foo: 5,
    bar: 6,
    bizz: 7
   };
   let test_foobar2 = FooBar2 {
    boo:8,
    far:9
   };
   println!("{}",test_foobar.get_foo());
   println!("{}",test_foobar2.get_far());
}
fn main() {
    println!("=============== RUN START ============");
    demo_declarative();
    //demo_derive();
    //demo_attribute();
    //demo_function_like();
}