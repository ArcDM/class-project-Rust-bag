// FILE: bagexamf64.rs

//! This module holds a variety of tests for the bagf64 structure
//!
//! 1) basic tests of insert and the constant member functions
//!
//! 2) tests of the copy constructor and == method
//!
//! 3) tests of the assignment operator
//!
//! 4) basic tests for the erase functions
//!
//! 5) basic tests for the += and + functions

extern crate rand;

use bagf64::Bag;

#[ doc( hidden ) ]
pub use bag::len_trait::{Len, Capacity, CapacityMut};

#[ test ]
///   Performs some basic tests of insert and the constant member functions. 
fn test1()
{
    #[ allow( non_snake_case ) ]
    let TEST_SIZE = 3000;
    let mut test_bag = Bag::new();

    // A. Testing size for an empty bag.
    assert_eq!( test_bag.len(), 0 );

    // B. Adding the number 4 to the bag, and then testing size.
    test_bag.insert( 4.0 );
    assert_eq!( test_bag.len(), 1 );

    // C. Inserting the number 2 into the bag. Then checking size
    test_bag.insert( 2.0 );
    assert_eq!( test_bag.len(), 2 );

    // D. Inserting the number 1 into the bag. Then checking size.
    test_bag.insert( 1.0 );
    assert_eq!( test_bag.len(), 3 );

    // E. Inserting the number 3 into the bag. Then checking size.
    test_bag.insert( 3.0 );
    assert_eq!( test_bag.len(), 4 );

    // F. Inserting another 2 into the bag. Then checking size.
    test_bag.insert( 2.0 );
    assert_eq!( test_bag.len(), 5 );

    //    Then checking occurrences of 2.
    assert_eq!( test_bag.occurrences( 2.0 ), 2 );

    // G. Inserting the numbers 5, 6, and 7 into the bag. Then checking size.
    test_bag.insert( 5.0 );
    test_bag.insert( 6.0 );
    test_bag.insert( 7.0 );
    assert_eq!( test_bag.len(), 8 );

    // H. Inserting two more 2's into the bag. And then checking occurrences of 2's.
    test_bag.insert( 2.0 );
    test_bag.insert( 2.0 );
    assert_eq!( test_bag.occurrences( 2.0 ), 4 );

    // I. Inserting TEST_SIZE random items between 0 and 49 and then checking size.

    for _ in 0..TEST_SIZE
    {
        test_bag.insert( ( rand::random::<f64>() % 50.0 ) );
    }

    assert_eq!( test_bag.len(), TEST_SIZE + 10 );
}

#[ test ]
///   Performs some tests of the copy constructor and == method.
fn test2()
{
    let mut test_bag = Bag::new();

    // A. Testing that copy constructor works okay for empty bag...
    let copy1 = test_bag.clone();
    assert_eq!( copy1.len(), 0 );

    // B. Testing copy constructor with 4-item bag...
    test_bag.insert( 1.0 );
    test_bag.insert( 1.0 );
    test_bag.insert( 1.0 );
    test_bag.insert( 1.0 );
    let copy2 = test_bag.clone();

    //    and now testing the == method...
    assert_eq!( test_bag, copy2 );
    assert_eq!( copy2, test_bag );

    test_bag.insert( 1.0 ); // Alter the original, but not the copy
    // C. Then checking size of copy.
    assert_eq!( copy2.len(), 4 );

    // D. Altering original but not the copy.
    assert_eq!( test_bag.len(), 5 );
}

#[ test ]
///   Performs some tests of the assignment operator.
fn test3()
{
    let mut test_bag = Bag::new();

    // A. Testing that assignment operator works okay for empty bag...
    let mut copy1 = Bag::new();
    copy1.insert( 1.0 );
    copy1 = test_bag.clone();

    assert_eq!( copy1.len(), 0 );

    // B. Testing assignment operator with 4-item bag...
    test_bag.insert( 1.0 );
    test_bag.insert( 1.0 );
    test_bag.insert( 1.0 );
    test_bag.insert( 1.0 );
    let mut copy2 = Bag::new();
    copy2.clone_from( &test_bag );
    test_bag.insert( 1.0 ); // Alter the original, but not the copy
    //    altering original by an insertion...

    assert_eq!( test_bag.occurrences( 1.0 ), 5 );
    assert_eq!( copy2.occurrences( 1.0 ), 4 );

    // Test passed.\n   testing size of assigned to...
    assert_eq!( copy2.len(), 4 );

    //    testing size of original...
    assert_eq!( test_bag.len(), 5 );
}

#[ test ]
///   Performs basic tests for the erase functions
fn test4()
{
    let mut test_bag = Bag::new();

    // Testing erase from empty bag, should have no effect.
    assert_eq!( test_bag.erase( 0.0 ), 0 );
    assert_eq!( test_bag.len(), 0 );
        
    // Inserting these: 8 6 10 1 7 10 15 3 13 2 5 11 14 4 12
    test_bag.insert(  8.0 );
    test_bag.insert(  6.0 );
    test_bag.insert( 10.0 );
    test_bag.insert(  1.0 );
    test_bag.insert(  7.0 );
    test_bag.insert( 10.0 );
    test_bag.insert( 15.0 );
    test_bag.insert(  3.0 );
    test_bag.insert( 13.0 );
    test_bag.insert(  2.0 );
    test_bag.insert(  5.0 );
    test_bag.insert( 11.0 );
    test_bag.insert( 14.0 );
    test_bag.insert(  4.0 );
    test_bag.insert( 12.0 );
    assert_eq!( test_bag.len(), 15 );

    // Now testing capacity, should be 16.
    assert_eq!( test_bag.capacity(), 16 );

    // Erasing 0, which is not in bag, so bag should be unchanged.
    assert!( !test_bag.erase_one( 0.0 ) );
    assert_eq!( test_bag.len(), 15 );

    // Erasing the 6.
    assert_eq!( test_bag.erase( 6.0 ), 1 );
    assert_eq!( test_bag.len(), 14 );

    // Erasing one 10.
    assert!( test_bag.erase_one( 10.0 ) );
    assert_eq!( test_bag.len(), 13 );

    // Erasing the 1.
    assert_eq!( test_bag.erase( 1.0 ), 1 );
    assert_eq!( test_bag.len(), 12 );

    // Erasing the 15.
    assert_eq!( test_bag.erase( 15.0 ), 1 );
    assert_eq!( test_bag.len(), 11 );

    // Erasing the 5.
    assert_eq!( test_bag.erase( 5.0 ), 1 );
    assert_eq!( test_bag.len(), 10 );

    // Erasing the 11.
    assert_eq!( test_bag.erase( 11.0 ), 1 );
    assert_eq!( test_bag.len(), 9 );

    // Erasing the 3.
    assert_eq!( test_bag.erase( 3.0 ), 1 );
    assert_eq!( test_bag.len(), 8 );

    // Erasing the 13.
    assert_eq!( test_bag.erase( 13.0 ), 1 );
    assert_eq!( test_bag.len(), 7 );

    // Erasing the 2.
    assert_eq!( test_bag.erase( 2.0 ), 1 );
    assert_eq!( test_bag.len(), 6 );

    // Erasing the one and only 14.
    assert!( test_bag.erase_one( 14.0 ) );
    assert_eq!( test_bag.len(), 5 );

    // Erasing the 4.
    assert_eq!( test_bag.erase( 4.0 ), 1 );
    assert_eq!( test_bag.len(), 4 );

    // Erasing the 12.
    assert_eq!( test_bag.erase( 12.0 ), 1 );
    assert_eq!( test_bag.len(), 3 );

    // Erasing the 8.
    assert_eq!( test_bag.erase( 8.0 ), 1 );
    assert_eq!( test_bag.len(), 2 );

    // Erasing the 7.
    assert_eq!( test_bag.erase( 7.0 ), 1 );
    assert_eq!( test_bag.len(), 1 );

    // Erasing the other 10.
    assert!( test_bag.erase_one( 10.0 ) );
    assert_eq!( test_bag.len(), 0 );

    // Testing capacity again.
    assert_eq!( test_bag.capacity(), 16 );

    // Now trimming to size.
    test_bag.shrink_to_fit();
    assert_eq!( test_bag.capacity(), 1 );

    // Now trimming to size again.
    test_bag.shrink_to_fit();
    assert_eq!( test_bag.capacity(), 1 );

    // Inserting value 5000 into the bag.
    // Inserting three 5's into the bag and then erasing all of them.
    test_bag.insert( 5000.0 );
    test_bag.insert( 5.0 ); 
    test_bag.insert( 5.0 );
    test_bag.insert( 5.0 );
    assert_eq!( test_bag.erase( 5.0 ), 3 );
    assert_eq!( test_bag.len(), 1 );
}

#[ test ]
///   Performs basic tests for the += and + functions
fn test5()
{
    let mut test_bag1 = Bag::new();
    let mut test_bag2 = Bag::new();
    let mut test_bag3 = Bag::new();

    // Inserting 2000 1's into test_bag1 and 2000 2's into test_bag2.

    for _ in 0..2000
    {
        test_bag1.insert( 1.0 );
        test_bag2.insert( 2.0 );
    }

    // Now testing the += function.
    test_bag1 += test_bag2.clone();

    //   and now testing for occurrences of 1's and 2's in test_bag1.
    assert_eq!( test_bag1.occurrences( 1.0 ), 2000 ); 
    assert_eq!( test_bag2.occurrences( 2.0 ), 2000 );

    // Now testing the + function.
    test_bag3.clone_from( &( test_bag2.clone() + test_bag2.clone() ) );

    //   and now testing for occurrences of 2's in test_bag3.
    assert_eq!( test_bag3.occurrences( 2.0 ), 4000 );
}
