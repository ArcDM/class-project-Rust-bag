// file main.rs

//! This project is made for the implementation
//! and testing of the bag structure.
//!
//! For the non-interactive test use `cargo test`
//!
//! For the interactive test use `cargo run`

#![ allow( dead_code ) ]
#![ allow( unused_imports ) ]

pub mod bag;
pub mod bagexam;

pub mod bagf64;
pub mod bagexamf64;

#[ allow( non_snake_case ) ]
pub mod bagVec;

pub mod bagtest;

fn main() {
    ::bagtest::bagtest();
}
