// FILE: bagtest.rs
// An interactive test program for the new bag class, implemented with
// a binary search tree.

use bag::Bag;
use std::ascii::AsciiExt;
use std::io::Write;

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
            'A' => { bag1 = bag2.clone(); }
            'a' => { bag2 = bag1.clone(); }
            'C' => { bag1 = copybag( &bag2 ); }
            'c' => { bag2 = copybag( &bag1 ); }
            'S' | 's' => { println!( "The bags' sizes are {} and {}",
                                        bag1.size(), bag2.size() ); }
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

fn print_menu()
// Postcondition: A menu of choices for this program has been written to cout.
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
    println!( "\t\t S  Print the result from the size( ) functions" );
    println!( "\t\t Q  Quit this test program" );
}

fn get_user_command() -> char
// Postcondition: The user has been prompted to enter a one character command.
// A line of input (with at least one character) has been read, and the first
// character of the input line is returned.
{
    print!( "\t\tEnter choice: " );

    read_input!().chars().next().expect( "Error: could not parse character" )
}

fn show_bags( bag1: &Bag<f32>, bag2: &Bag<f32> )
// Postcondition: The function has tested whether the numbers 0..9 are in
// the two bags, and printed the results to standard output.
{
    println!( "\t\tbag1 {:?}\n\t\tbag2 {:?}",
                bag1, bag2 );
}

fn copybag( input_bag: &Bag<f32> ) -> Bag<f32>
// Postcondition: The return value is a copy of b.
{
    input_bag.clone()
}

fn get_number() -> f32
// Postcondition: The user has been prompted to enter bag::value_type  number. The
// number has been read, echoed to the screen, and returned by the function.
{
    print!( "\t\tPlease enter a bag::value_type number for the bag: ");
    let result: f32 = read_input!().trim_right_matches( '\n' ).parse::<f32>().expect( "Error: could not parse f32" );
    println!( "\t\t{} has been read.", result );
    result
}
