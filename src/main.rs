#![ allow( dead_code ) ]
mod bag;
mod bagtest;
//pub mod bagf64;

#[cfg(test)]
mod bagexam;

fn main() {
    ::bagtest::bagtest();
}
