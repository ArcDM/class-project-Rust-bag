// FILE: bagtest.rs

//! An interactive test program for the bag generic structure.
//!
//! Run with `cargo run`
//!
//! # The following commands are available with 2 bags
//!
//! * `A` Use the assignment operator to make bag1 equal to bag2
//!
//! * `a` Use the assignment operator to make bag2 equal to bag1
//!
//! * `C` Use the copy constructor to make bag1 equal to bag2
//!
//! * `c` Use the copy constructor to make bag2 equal to bag1
//!
//! * `I` Insert an item into bag1
//!
//! * `i` Insert an item into bag2
//!
//! * `R` Erase all of item from bag1
//!
//! * `r` Erase all of item from bag2
//!
//! * `X` Erase one of item from bag1
//!
//! * `x` Erase one of item from bag2
//!
//! * `O` Display both bags
//!
//! * `S` Print the result from the len( ) functions
//!
//! * `Q` Quit this test program

use bag::Bag;

#[ allow( unused_imports ) ]
use bag::len_trait::{Len, Capacity, CapacityMut};
use std::io::Write;
use std::ascii::AsciiExt;

macro_rules! read_input
{
    () =>
    {{
        ::std::io::stdout().flush().unwrap();
        let mut input = String::new();
        ::std::io::stdin().read_line( &mut input ).expect( "Error: could not read input" );
        input
    }};
}

pub fn bagtest()
{
    // bags that we'll perform tests on
    let mut bag1: Bag<f32> = Bag::new();
    let mut bag2: Bag<f32> = Bag::new();

    let mut choice; // A command character entered by the user

    println!( "\tI have initialized two empty bags of doubles." );


    while
    {
        print_menu();
        choice = get_user_command();

        match choice
        {
            'A' => { bag1 = copybag( &bag2 ); }
            'a' => { bag2 = copybag( &bag1 ); }
            'C' => { bag1.clone_from( &bag2 ); }
            'c' => { bag2.clone_from( &bag1 ); }
            'S' | 's' => { println!( "The bags' sizes are {} and {}",
                                        bag1.len(), bag2.len() ); }
            'I' => { bag1.insert( get_number() ); }
            'i' => { bag2.insert( get_number() ); }
            'R' => { bag1.erase( get_number() ); }
            'r' => { bag2.erase( get_number() ); }
            'X' => { bag1.erase_one( get_number() ); }
            'x' => { bag2.erase_one( get_number() ); }
            'O' | 'o' => { show_bags( &bag1, &bag2 ); }
            'q' | 'Q' => { println!( "Ridicule is the best test of truth." ); }
            val => { println!( "{} is invalid. Sorry.", val ); }
        }

        !choice.eq_ignore_ascii_case( &'Q' ) // post check
    }{}
}

// A menu of choices for this program has been written to cout.
fn print_menu()
{
    println!( "\n\n\tThe following choices are available with 2 bags: " );
    println!( "\t\t A  Use the assignment operator to make bag1 equal to bag2" );
    println!( "\t\t a  Use the assignment operator to make bag2 equal to bag1" );
    println!( "\t\t C  Use the copy constructor to make bag1 equal to bag2" );
    println!( "\t\t c  Use the copy constructor to make bag2 equal to bag1" );
    println!( "\t\t I  Insert an item into bag1" );
    println!( "\t\t i  Insert an item into bag2" );
    println!( "\t\t R  Erase all of item from bag1" );
    println!( "\t\t r  Erase all of item from bag2" );
    println!( "\t\t X  Erase one of item from bag1" );
    println!( "\t\t x  Erase one of item from bag2" );
    println!( "\t\t O  Display both bags" );
    println!( "\t\t S  Print the result from the len( ) functions" );
    println!( "\t\t Q  Quit this test program" );
}

// The user has been prompted to enter a one character command.
// A line of input (with at least one character) has been read,
// and the first character of the input line is returned.
fn get_user_command() -> char
{
    print!( "\t\tEnter choice: " );

    read_input!().chars().next().expect( "Error: could not parse character" )
}

// The function has tested whether the numbers 0..9 are in
// the two bags, and printed the results to standard output.
fn show_bags( bag1: &Bag<f32>, bag2: &Bag<f32> )
{
    println!( "\t\tbag1 {:?}\n\t\tbag2 {:?}",
                bag1, bag2 );
}

// The return value is a copy of b.
fn copybag( input_bag: &Bag<f32> ) -> Bag<f32>
{
    input_bag.clone()
}

// The user has been prompted to enter bag::value_type  number. The number
// has been read, echoed to the screen, and returned by the function.
fn get_number() -> f32
{
    print!( "\t\tPlease enter a bag::value_type number for the bag: ");
    let result: f32 = read_input!().trim_right_matches( '\n' ).parse::<f32>().expect( "Error: could not parse f32" );
    println!( "\t\t{} has been read.", result );
    result
}
