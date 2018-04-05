// FILE: bagexam.rs
// Non-interactive test program for the bag class
//
// DESCRIPTION:
// Each function tests part of the bag class, returning some
// number of points to indicate how much of the test was passed.
// A description and result of each test is printed to cout.
// Maximum number of points awarded by this program is determined by the
// constants POINTS[1], POINTS[2]...
//

#![ allow( unused_assignments ) ]
#![ allow( dead_code ) ]

extern crate rand;

use bag::Bag;
use std::io::{self, Write};

macro_rules! print_test {
    ( $( $args:tt )* ) => 
    (
        let mut output = String::new();
        ::std::fmt::write( &mut output, format_args!( $( $args )* ) ).unwrap();
        io::stdout().write( output.as_bytes() ).unwrap();
    );
}

macro_rules! println_test {
    () => ( print_test!( "\n" ) );
    ( $fmt:expr ) => ( print_test!( concat!( $fmt, "\n" ) ) );
    ( $fmt:expr, $( $arg:tt )* ) => ( print_test!( concat!( $fmt, "\n" ), $( $arg )* ) );
}

macro_rules! read_input {
    () => 
    {{
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line( &mut input ).unwrap();
        input
    }};
}

#[ test ]
#[ ignore ]
fn dont_ask()
{
    main_( false );
}

#[ test ]
fn bagexam()
{
    main_( true );
}

// Descriptions and points for each of the tests:
const MANY_TESTS: u8 = 5;
const POINTS: [ u8; 6 ] = [
    100, // Total points for all tests.
     32,  // Test 1 points
     12,  // Test 2 points
     12,  // Test 3 points
     32,  // Test 4 points
     12   // Test 5 points
];

const DESCRIPTION: [ &'static str; 6 ] = [
    "tests for bag Class",
    "Testing insert and the constant member functions",
    "Testing the copy constructor and == methodr",
    "Testing the assignment operator",
    "Testing erase and erase_one functions",
    "Testing += method and non-instance method +"
];

/// **************************************************************************
///   This function determines if the bag (test) is "correct" according to
///   this requirement:
///   a. it has exactly n items.
/// **************************************************************************
fn correct<Type: PartialEq + Clone + Default>( test_bag: &Bag<Type> , n: usize ) -> bool
{
    if test_bag.size( ) == n
    {
        println_test!( "Test passed.\n" );
        true
        
    }
    else
    {
        println_test!( "Test failed.\n" );
        false
    }
}


/// **************************************************************************
////   Performs some basic tests of insert and the constant member
///   functions. Returns POINTS[1] if the tests are passed. Otherwise returns 0.
/// **************************************************************************
fn test1() -> u8
{
    #[ allow( non_snake_case ) ]
    let TEST_SIZE = 3000;
    let mut test_bag = Bag::new();
    let mut test_letter = 'A' as u8;

    println_test!( "{}. Testing size for an empty bag.", test_letter as char );
    test_letter += 1;

    if !correct( &test_bag, 0 )
    {
        return 0;
    }


    println_test!( "{}. Adding the number 4 to the bag, and then testing\n   size.", test_letter as char );
    test_letter += 1;
    test_bag.insert( &4 );

    if !correct( &test_bag, 1 )
    {
        return 0;
    }


    println_test!( "{}. Inserting the number 2 into the bag.\n   Then checking size.", test_letter as char );
    test_letter += 1;
    test_bag.insert( &2 );


    if !correct( &test_bag, 2 )
    {
        return 0;
    }


    println_test!( "{}. Inserting the number 1 into the bag.\n   Then checking size.", test_letter as char );
    test_letter += 1;
    test_bag.insert( &1 );

    if !correct( &test_bag, 3 )
    {
        return 0;
    }


    println_test!( "{}. Inserting the number 3 into the bag.\n   Then checking size.", test_letter as char );
    test_letter += 1;
    test_bag.insert( &3 );

    if !correct( &test_bag, 4 )
    {
        return 0;
    }


    println_test!( "{}. Inserting another 2 into the bag.\n   Then checking size.", test_letter as char );
    test_letter += 1;
    test_bag.insert( &2 );

    if !correct( &test_bag, 5 )
    {
        return 0;
    }

    println_test!( "   Then checking occurrences of 2." );

    if test_bag.occurrences( &2 ) == 2
    {
        println_test!( "Test passed." );
    }
    else
    {
        return 0;
    }


    println_test!( "{}. Inserting the numbers 5, 6, and 7 into the bag.\n   Then checking size.", test_letter as char );
    test_letter += 1;
    test_bag.insert( &5 );
    test_bag.insert( &6 );
    test_bag.insert( &7 );

    if !correct( &test_bag, 8 )
    {
        return 0;
    }


    println_test!( "{}. Inserting two more 2's into the bag.\n   and then checking occurrences of 2's\n", test_letter as char );
    test_letter += 1;
    test_bag.insert( &2 );
    test_bag.insert( &2 );

    if test_bag.occurrences( &2 ) == 4
    {
        println_test!( "Test passed." );
    }
    else
    {
        return 0;
    }


    println_test!( "{}. Inserting {} random items between 0 and 49\n   and then checking size.", test_letter as char, TEST_SIZE );

    for _ in 0..TEST_SIZE
    {
        test_bag.insert( &( rand::random::<i16>() % 50 ) );
    }

    if !correct( &test_bag, TEST_SIZE + 10 )
    {
        return 0;
    }


    POINTS[ 1 ]
}



/// **************************************************************************
///   Performs some tests of the copy constructor and == method.
///   Returns POINTS[2] if the tests are passed. Otherwise returns 0.
/// **************************************************************************
fn test2() -> u8
{
    let mut test_bag = Bag::new();

    println_test!( "A. Testing that copy constructor works okay for empty bag..." );
    let copy1 = Bag::from_bag( &test_bag );

    if !correct( &copy1, 0 )
    {
        return 0;
    }


    print_test!( "B. Testing copy constructor with 4-item bag..." );
    test_bag.insert( &1 );
    test_bag.insert( &1 );
    test_bag.insert( &1 );
    test_bag.insert( &1 );
    let copy2 = Bag::from_bag( &test_bag );
    println_test!( "   and now testing the == method..." );

    if !( test_bag == copy2 ) || !( copy2 == test_bag )
    {
        println_test!( "Test failed.\n" );
        return 0;
    }
    else
    {
        println_test!( "Test passed.\n" );
    }


    test_bag.insert( &1 ); // Alter the original, but not the copy
    println_test!( "C. Then checking size of copy" );

    if !correct( &copy2, 4 )
    {
        return 0;
    }

    println_test!( "D. Altering original but not the copy" );

    if !correct( &test_bag, 5 )
    {
        return 0;
    }


    println_test!( "Copy constructor seems okay." );
    POINTS[ 2 ]
}


/// **************************************************************************
///   Performs some tests of the assignment operator.
///   Returns POINTS[3] if the tests are passed. Otherwise returns 0.
/// **************************************************************************
fn test3() -> u8
{
    let mut test_bag = Bag::new();
    let oldbytes;
    let newbytes;

    println_test!( "A. Testing that assignment operator works okay for empty bag..." );
    let mut copy1 = Bag::new();
    copy1.insert( &1 );
    copy1 = test_bag.clone();

    if !correct( &copy1, 0 )
    {
        return 0;
    }


    println_test!( "B. Testing assignment operator with 4-item bag..." );
    test_bag.insert( &1 );
    test_bag.insert( &1 );
    test_bag.insert( &1 );
    test_bag.insert( &1 );
    let mut copy2 = Bag::new();
    copy2 = test_bag.clone();
    test_bag.insert( &1 ); // Alter the original, but not the copy
    println_test!( "   altering original by an insertion..." );

    if test_bag.occurrences( &1 ) != 5 || copy2.occurrences( &1 ) != 4
    {
        println_test!( "Test failed." );
        return 0;
    }

    println_test!( "Test passed.\n   testing size of assigned to..." );

    if !correct( &copy2, 4 )
    {
        return 0;
    }

    println_test!( "   testing size of original..." );

    if !correct( &test_bag, 5 )
    {
        return 0;
    }


    print_test!( "C. Testing assignment operator for a self-assignment..." );

    let mut hasher = ::std::collections::hash_map::DefaultHasher::new();
    oldbytes = ::std::hash::Hash::hash( &test_bag, &mut hasher );
    test_bag = test_bag;
    newbytes = ::std::hash::Hash::hash( &test_bag, &mut hasher );


    if oldbytes == newbytes
    {
        println_test!( "passed." );
    }
    else
    {
        println_test!( "failed." );
        return 0;
    }


    println_test!( "Assignment operator seems okay." );
    POINTS[ 3 ]
}


/// **************************************************************************
///   Performs basic tests for the erase functions
///   Returns POINTS[4] if the tests are passed.
///   Otherwise returns 0.
/// **************************************************************************
fn test4() -> u8
{
    let mut test_bag = Bag::new();

    print_test!( "Testing erase from empty bag (should have no effect) ..." );
    test_bag.erase( &0 );

    if !correct( &test_bag, 0 )
    {
        return 0;
    }

        
    println_test!( "Inserting these: 8 6 10 1 7 10 15 3 13 2 5 11 14 4 12" );
    test_bag.insert( & 8 );
    test_bag.insert( & 6 );
    test_bag.insert( &10 );
    test_bag.insert( & 1 );
    test_bag.insert( & 7 );
    test_bag.insert( &10 );
    test_bag.insert( &15 );
    test_bag.insert( & 3 );
    test_bag.insert( &13 );
    test_bag.insert( & 2 );
    test_bag.insert( & 5 );
    test_bag.insert( &11 );
    test_bag.insert( &14 );
    test_bag.insert( & 4 );
    test_bag.insert( &12 );

    if !correct( &test_bag, 15 )
    {
        return 0;
    }


    println_test!("Now testing capacity -- should be 16" );

    if test_bag.get_capacity() == 16
    {
        println_test!( "Test passed.\n" );
    }
    else
    {
        println_test!( "Test failed." );
        println_test!( "{:?}\n", test_bag );
        return 0;
    }


    println_test!( "Erasing 0 (which is not in bag, so bag should be unchanged) ..." );

    if test_bag.erase_one( &0 )
    {
        println_test!( "Test failed" );
        return 0;
    }

    if !correct( &test_bag, 15 )
    {
        return 0;
    }


    print_test!( "Erasing the 6 ..." );
    test_bag.erase( &6 );

    if !correct( &test_bag, 14 )
    {
        return 0;
    }


    print_test!( "Erasing one 10 ..." );

    if !test_bag.erase_one( &10 )
    {
        println_test!( "Test failed" );
        return 0;
    }

    if !correct( &test_bag, 13 )
    {
        return 0;
    }


    print_test!( "Erasing the 1 ..." );
    test_bag.erase( &1 );

    if !correct( &test_bag, 12 )
    {
        return 0;
    }


    print_test!( "Erasing the 15 ..." );
    test_bag.erase( &15 );

    if !correct( &test_bag, 11 )
    {
        return 0;
    }


    print_test!( "Erasing the 5 ..." );
    test_bag.erase( &5 );

    if !correct( &test_bag, 10 )
    {
        return 0;
    }


    print_test!( "Erasing the 11 ..." );
    test_bag.erase( &11 );

    if !correct( &test_bag, 9 )
    {
        return 0;
    }


    print_test!( "Erasing the 3 ..." );
    test_bag.erase( &3 );

    if !correct( &test_bag, 8 )
    {
        return 0;
    }


    print_test!( "Erasing the 13 ..." );
    test_bag.erase( &13 );

    if !correct( &test_bag, 7 )
    {
        return 0;
    }


    print_test!( "Erasing the 2 ..." );
    test_bag.erase( &2 );

    if !correct( &test_bag, 6 )
    {
        return 0;
    }


    print_test!( "Erasing the one and only 14 ..." );
    test_bag.erase_one( &14 );

    if !correct( &test_bag, 5 )
    {
        return 0;
    }


    print_test!( "Erasing the 4 ..." );
    test_bag.erase( &4 );

    if !correct( &test_bag, 4 )
    {
        return 0;
    }


    print_test!( "Erasing the 12 ..." );
    test_bag.erase( &12 );

    if !correct( &test_bag, 3 )
    {
        return 0;
    }


    print_test!( "Erasing the 8 ..." );
    test_bag.erase( &8 );

    if !correct( &test_bag, 2 )
    {
        return 0;
    }


    print_test!( "Erasing the 7 ..." );
    test_bag.erase( &7 );

    if !correct( &test_bag, 1 )
    {
        return 0;
    }


    print_test!( "Erasing the other 10 ..." );

    if !test_bag.erase_one( &10 )
    {
        println_test!( "Test failed ..." );
        return 0;
    }

    if !correct( &test_bag, 0 )
    {
        return 0;
    }


    print_test!( "Testing capacity again..." );

    if test_bag.get_capacity() != 16
    {
        println_test!( "Test failed.\n{:?}\n", test_bag );
        return 0;
    }


    println_test!( "Now trimming to size" );
    test_bag.trim_to_size();

    if test_bag.get_capacity() != 1
    {
        println_test!( "Test failed.\n\n{:?}", test_bag );
        return 0;
    }

    println_test!( "Test passed.\n" );


    println_test!( "Now trimming to size again" );
    test_bag.trim_to_size();

    if test_bag.get_capacity() != 1
    {
        println_test!( "Test failed.\n\n{:?}", test_bag );
        return 0;
    }

    println_test!( "Test passed.\n" );


    print_test!( "Inserting value 5000 into the bag ... \nInserting three 5's into the bag and then erasing all of them ..." );
    test_bag.insert( &5000 );
    test_bag.insert( &5 ); 
    test_bag.insert( &5 );
    test_bag.insert( &5 );
    test_bag.erase( &5 );

    if !correct( &test_bag, 1 )
    {
        return 0;
    }

    
    println_test!( "Erase functions seem okay." );
    POINTS[ 4 ]
}

/// **************************************************************************
///   Performs basic tests for the += and + functions
///   Returns POINTS[5] if the tests are passed.
///   Otherwise returns 0.
/// **************************************************************************
fn test5() -> u8
{
    let mut test_bag1 = Bag::new();
    let mut test_bag2 = Bag::new();
    let mut test_bag3 = Bag::new();


    println_test!( "Inserting 2000 1's into test_bag1 and 2000 2's into test_bag2" );

    for _ in 0..2000
    {
        test_bag1.insert( &1 );
        test_bag2.insert( &2 );
    }

    println_test!( "Now testing the += function ..." );
    test_bag1 += test_bag2.clone();
    println_test!( "  and now testing for occurrences of 1's and 2's in test_bag1 ..." );

    if test_bag1.occurrences( &1 ) == 2000 && test_bag2.occurrences( &2 ) == 2000
    {
        println_test!( "Test passed.\n" );
    }
    else
    {
        println_test!( "Test failed.\n" );
        return 0;
    }


    println_test!( "Now testing the + function ..." );
    test_bag3 = test_bag2.clone() + test_bag2.clone();
    println_test!( "  and now testing for occurrences of 2's in test_bag3 ..." );

    if test_bag3.occurrences( &2 ) == 4000
    {
        println_test!( "Test passed.\n" );
    }
    else
    {
        println_test!( "Test failed.\n" );
        return 0;
    }


    println_test!( "+= and + functions seem okay." );
    POINTS[ 5 ]
}

fn run_a_test( number: u8, message: &str, test_function: fn() -> u8, max: u8 ) -> u8
{
    let result;
    
    println_test!( "\nSTART OF TEST {}:\n{} ({} points).", number, message, max );
    result = test_function();

    if result > 0
    {
        println_test!( "Test {} got {} points out of a possible {}.", number, result, max );
    }
    else
    {
        println_test!( "Test {} failed.", number );
    }

    println_test!( "END OF TEST {}.\n", number );    

    result
}


/// **************************************************************************
///   Calls all tests and prints the sum of all points
///   earned from the tests.
/// **************************************************************************
fn main_( take_input: bool )
{
    let mut sum = 0;
    let mut done_erase = false;
    let mut done_union = false;
    
    println_test!( "Running {}", DESCRIPTION[ 0 ] );

    if take_input
    {
        print_test!( "Have you implemented erase yet? [Y or N]: " );

        let mut line = read_input!();

        if line == "Y\n" || line == "y\n"
        {
            done_erase = true;
        }

        print_test!( "Have you implemented += and + yet? [Y or N]: " );

        line = read_input!();

        if line == "Y\n" || line == "y\n"
        {
            done_union = true;
        }
    }


    sum += run_a_test( 1, DESCRIPTION[ 1 ], test1, POINTS[ 1 ] );
    sum += run_a_test( 2, DESCRIPTION[ 2 ], test2, POINTS[ 2 ] );
    sum += run_a_test( 3, DESCRIPTION[ 3 ], test3, POINTS[ 3 ] );

    if done_erase || !take_input
    {
        sum += run_a_test( 4, DESCRIPTION[ 4 ], test4, POINTS[ 4 ] );
    }
    if done_union || !take_input
    {
        sum += run_a_test( 5, DESCRIPTION[ 5 ], test5, POINTS[ 5 ] );
    }


    println_test!( "If you submit your bag to Prof. Haiduk now, you will have\n{} points out of the {} points from this test program.", sum, POINTS[ 0 ] );

    assert!( sum == POINTS[ 0 ] );
}
