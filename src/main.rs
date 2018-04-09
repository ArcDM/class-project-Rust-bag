// file main.re
#![ allow( dead_code ) ]

mod bag;
#[cfg(test)]
mod bagexam;

mod bagf64;
#[cfg(test)]
mod bagexamf64;

#[ allow( non_snake_case ) ]
pub mod bagVec;

fn main() {}
