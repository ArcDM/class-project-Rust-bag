// file main.rs
#![ allow( dead_code ) ]

mod bag;
#[cfg(test)]
mod bagexam;

mod bagf64;
#[cfg(test)]
mod bagexamf64;

mod bagtest;

fn main() {
    ::bagtest::bagtest();
}
