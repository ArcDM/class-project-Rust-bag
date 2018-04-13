#![ allow( dead_code ) ]

pub mod bag;
pub mod bagf64;

#[ allow( non_snake_case ) ]
pub mod bagVec;

pub mod bagtest;

fn main() {
    ::bagtest::bagtest();
}
