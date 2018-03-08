
#![ allow( dead_code ) ]
mod bag;
mod bagtest;

#[cfg(test)]
mod bagexam;

fn main() {
    ::bagtest::bagtest();
}
