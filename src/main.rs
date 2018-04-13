// file main.rs
#![ allow( dead_code ) ]

pub mod bag;
pub mod bagexam;

pub mod bagf64;
pub mod bagexamf64;

pub mod bagVec;

mod bagtest;

fn main() {
    ::bagtest::bagtest();
}
