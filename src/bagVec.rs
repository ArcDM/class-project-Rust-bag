// File: bag.rs 

//! A Bag as an unsorted, ordered collection of generic types in which
//! the same item may appear multiple times. The bag's capacity can
//! grow as needed and can be reduced.
//!
//! This varient of the bag take advantage of the methods that the
//! Vector structure provides. Unfortunately using the vector's variable
//! for capacity give its methods complete control over its changes,
//! making for unforeseeable results when using its capacity.
//!
//! The type used must have the traits `PartialEq` and `Clone`.
//!
//! # Note
//! Because of the slow linear algorithms of this
//! class, large bags will have poor performance.
//
//  # Author
//  Nathan Bradley based on the java implementation from
//  H. Paul Haiduk with credit given to Michael Main
//
//  # Version
//  8.April.2018

#![ allow( dead_code ) ]

pub extern crate len_trait;
use std::cmp::PartialEq;

/// A container for inserting and removing given values.
///
/// # Invariant of the Bag struct:
///
/// 1. The number of elements in the bag is no more than data.len().
///
/// 2. For an empty bag, we do not care what is stored in any of data;
/// for a non-empty bag, the elements in the bag are stored in `data[ 0 ]`
/// through `data[ data.len() - 1 ]`, and we don't care what's in the
/// rest of data.

pub struct Bag<Type: PartialEq + Clone>
{
   data: Vec<Type>
}

impl<Type: PartialEq + Clone> Bag<Type>
{
    /// Initialize an empty bag.
    ///
    /// # Return
    /// A bag that is empty and has a capacity of one.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocating a new array.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// let mut bag = Bag::new();
    /// ```

    pub fn new() -> Self
    {
        Bag { data: Vec::with_capacity( 1 ) }
    }

    /// Erase all copies of a specified element from this bag if target exists in bag.
    ///
    /// # Parameter: `target`
    /// The element(s) to remove from the bag.
    ///
    /// # Postcondition
    /// If `target` was found in the bag, then all copies of
    /// `target` have been removed and the method returns number of items removed.
    ///
    /// The length will change if one is found, but the capacity will not.
    ///
    /// # Return
    /// An unsigned integer value representing the number of items erased from bag.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut bag = Bag::new();
    /// bag.insert( 1 );
    /// bag.insert( 2 );
    /// bag.insert( 2 );
    /// bag.insert( 3 );
    /// bag.insert( 3 );
    /// bag.insert( 3 );
    ///
    /// assert_eq!( bag.erase( 2 ), 2 );
    /// assert_eq!( bag.len(), 4 );
    /// ```

    pub fn erase( &mut self, target: Type ) -> usize
    {
        let old_len = self.data.len();
        let mut index = 0;

        // loop will only iterate though the full
        //  range of 0 to this.len(), which can vary though the loop
        while index < self.data.len()
        {
            if self.data[ index ] == target
            {
               self.data.swap_remove( index );
            }
            else
            {
               index += 1;
            }
        }

        old_len - self.data.len()
    }

    /// Remove one copy of a specified element from this bag.
    ///
    /// # Parameter: `target`
    /// The element to remove from the bag.
    ///
    /// # Postcondition
    /// If `target` was found in the bag, then only the first copy of
    /// `target` has been removed and the method returns true. 
    /// Otherwise the bag remains unchanged and the method returns false.
    ///
    /// The length will change if one is found, but the capacity will not.
    ///
    /// # Return
    /// True or false depending on whether `target` exists in the bag.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut bag = Bag::new();
    /// bag.insert( 1 );
    /// bag.insert( 2 );
    /// bag.insert( 3 );
    ///
    /// assert!( bag.erase( 2 ) );
    /// assert_eq!( bag.len(), 2 );
    /// ```

    pub fn erase_one( &mut self, target: Type ) -> bool
    {
        match self.data.iter().position( | value | *value == target )
        {
            None    =>      false,
            Some( index ) =>  
                {
                    self.data.swap_remove( index );
                    true
                }
        }
    }

    /// Add a new element to this bag doubling capacity if needed.
    ///
    /// # Parameter: `new_item`
    /// The new element that is being inserted.
    ///
    /// # Postcondition
    /// A new copy of the element has been added to this bag.
    ///
    /// The length will increase by one, the capacity only change if needed.
    ///
    /// # Caution
    ///   Because of how reserve_exact() works, this method does
    ///   not always work as intended.
    ///
    /// # Panics
    /// If `self.data.capacity() * 2` causes an unsigned integer overflow.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocating a new array.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut bag = Bag::new();
    ///
    /// bag.insert( 2 );
    /// bag.insert( 4 );
    /// bag.insert( 6 );
    /// assert_eq!( bag.len(), 3 );
    ///
    /// bag.insert( 3 );
    /// assert_eq!( bag.len(), 4 );
    /// ```

    pub fn insert( &mut self, new_item: Type )
    {
        if self.data.len() == self.data.capacity()
        {
            let extra_capacity = self.data.capacity();
            self.data.reserve_exact( extra_capacity );
        }

        self.data.push( new_item );
    } 

    /// Accessor method to count the number of occurrences of a
    /// particular element in this bag.
    ///
    /// # Parameter: `target`
    /// The element for which number of occurrences will be counted.
    ///
    /// # Postcondition
    /// This method does not alter state of the bag.
    ///
    /// # Return
    /// The number of times that `target` occurs in this bag.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut bag = Bag::new();
    /// bag.insert( 2 );
    /// bag.insert( 4 );
    /// bag.insert( 4 );
    /// bag.insert( 6 );
    /// bag.insert( 6 );
    /// bag.insert( 6 );
    ///
    /// assert_eq!( bag.occurrences( 4 ), 2 );
    /// ```

    pub fn occurrences( &self, target: Type ) -> usize
    {
        self.data.iter()
                .filter( | &value | *value == target )
                .count()
    }
}

impl<Type: PartialEq + Clone> len_trait::len::Len for Bag<Type>
{
    /// Determine the number of elements in this bag.
    ///
    /// # Postcondition
    /// This method does not alter state of the bag.
    ///
    /// # Return
    /// The number of elements in this bag.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut bag = Bag::new();
    /// bag.insert( 2 );
    /// bag.insert( 4 );
    /// bag.insert( 6 );
    ///
    /// assert_eq!( bag.len(), 3 );
    /// ```

    fn len( &self ) -> usize
    {
        self.data.len()
    }
}

impl<Type: PartialEq + Clone> len_trait::len::Empty for Bag<Type>
{
    /// Determine if the bag is empty.
    ///
    /// # Postcondition
    /// This method does not alter state of the bag.
    ///
    /// # Return
    /// True if the bag is empty, else false.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut bag = Bag::new();
    /// assert!( bag.is_empty() );
    ///
    /// bag.insert( 1 );
    /// assert!( !bag.is_empty() );
    /// ```

    fn is_empty( &self ) -> bool
    {
        self.data.is_empty()
    }
}

impl<Type: PartialEq + Clone> len_trait::len::Clear for Bag<Type>
{
    /// Empty the bag.
    ///
    /// # Postcondition
    /// The capacity of the bag is not altered.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut bag = Bag::new();
    /// bag.insert( 1 );
    /// bag.insert( 2 );
    /// bag.insert( 3 );
    /// assert!( !bag.is_empty() );
    ///
    /// bag.clear();
    /// assert!( bag.is_empty() );

    fn clear(&mut self)
    {
        self.data.clear();
    }
}

impl<Type: PartialEq + Clone> len_trait::capacity::Capacity for Bag<Type>
{
    /// Return the current capacity of the bag.
    ///
    /// # Postcondition
    /// This method does not alter state of the bag.
    ///
    /// # Return
    /// An unsigned integer that represents total capacity of this bag.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut bag = Bag::with_capacity( 10 );
    /// assert_eq!( bag.capacity(), 10 );
    /// ```

    fn capacity( &self ) -> usize
    {
        self.data.capacity()
    }
}

impl<Type: PartialEq + Clone> len_trait::capacity::WithCapacity for Bag<Type>
{
    /// Initialize an empty bag having a capacity of `initial_capacity`.
    ///
    /// # Parameter: `initial_capacity`
    /// An unsigned integer greater than 0.
    ///
    /// # Precondition
    /// `initial_capacity` must be greater than 0.
    ///
    /// # Return
    /// A bag that is empty and has a capacity of `initial_capacity`.
    ///
    /// # Panics
    /// `initial_capacity` given is not greater than 0.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocating a new array.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut bag = Bag::with_capacity( 10 );
    ///
    /// // The bag contains no items, even though it has capacity for more
    /// assert_eq!( bag.len(), 0 );
    ///
    /// // These are all done without increasing capacity...
    /// for i in 0..10
    /// {
    ///     bag.push( i );
    /// }
    ///
    /// // ...but this will cause the capacity to increase
    /// bag.insert( 11 );
    /// ```

    fn with_capacity( initial_capacity: usize ) -> Self
    {
        if initial_capacity <= 0
        {
            panic!( "initialCapacity must be > 0" );
        }
        else
        {
            Bag { data: Vec::with_capacity( initial_capacity ) }
        }
    }
}

impl<Type: PartialEq + Clone> len_trait::capacity::CapacityMut for Bag<Type>
{
    /// Potentially increase capacity of this bag.
    ///
    /// # Parameter: `new_capacity`
    /// An unsigned integer greater than 0.
    ///
    /// # Precondition
    /// new_capacity must be greater than 0.
    ///
    /// # Postcondition
    /// The bag's capacity is at least `new_capacity`.  If the capacity
    /// was already at or greater than `new_capacity`, then the capacity
    /// is left unchanged.
    ///
    /// # Note
    /// Unlike the expected reserve trait that takes an argument
    /// of additional capacity, this implementation takes a total
    /// new capacity as the argument.
    ///
    /// # Caution
    /// Because of how reserve_exact() works, this method does
    /// not always work as intended.
    ///
    /// # Panics
    /// `new_capacity` given is not greater than 0.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocating a new array.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut bag = Bag::new();
    /// bag.reserve( 10 );
    ///
    /// assert_eq!( bag.capacity(), 10 );
    /// ```

    fn reserve( &mut self, mut new_capacity: usize )
    {
        if new_capacity <= 0
        {
            panic!( "new_capacity < 1" );
        }
        else
        {
            if self.data.capacity() < new_capacity
            {
                new_capacity -= self.data.capacity();
                self.data.reserve_exact( new_capacity );
            }
        }
    }

    /// Reduces the capacity of this bag to current size if there is
    /// excess capacity.
    ///
    /// # Postcondition
    /// Capacity of this bag is reduced to the current number
    /// of items in bag or left unchanged if capacity equals to
    /// number of items in bag but must be at least one.
    ///
    /// # Caution
    /// Because of how shrink_to_fit() works, this method does
    /// not always work as intended.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut bag = Bag::with_capacity( 10 );
    /// bag.insert( 1 );
    /// bag.insert( 2 );
    /// bag.insert( 3 );
    /// assert_eq!( bag.capacity(), 10 );
    ///
    /// bag.shrink_to_fit();
    /// assert_eq!( bag.capacity(), 3 );
    /// ```

    fn shrink_to_fit( &mut self )
    {
        self.data.shrink_to_fit();

        if self.data.len() <= 1
        {
            self.data.reserve_exact( 1 );
        }
    }
}

impl<Type: PartialEq + Clone> Default for Bag<Type>
{
    /// Initialize an empty bag.
    ///
    /// # Return
    /// A bag that is empty and has a capacity of one.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocating a new array.

    fn default() -> Self
    {
        Bag { data: Vec::with_capacity( 1 ) }
    }
}

impl<Type: PartialEq + Clone> Clone for Bag<Type>
{
    /// Generate a copy of this bag.
    ///
    /// # Postcondition
    /// * The clone is not the same object as the source.
    ///
    /// * The clone is the same type as the source.
    ///
    /// * The clone will equal the source as long as both
    /// bags are unaltered.
    ///
    /// # Return
    /// A new bag initialized as bag with all the elements in
    /// source and with capacity to equal that number of elements.
    ///
    /// Subsequent changes to the copy will not affect the original,
    /// nor vice versa.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocating a new array.

    fn clone( &self ) -> Self
    {
        Bag { data: self.data.clone() }
    }

    /// Make a copy from a source.
    ///
    /// # Parameter: `source`
    /// A reference to a bag that is to be copied.
    ///
    /// # Postcondition
    /// This bag is now a copy of `source` and has a capacity of
    /// of number of elements in `source`. Subsequent changes to
    /// the copy will not affect the original, nor vice versa.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocating a new array.

    fn clone_from( &mut self, source: &Self )
    {
        self.data = source.data.clone();
    }
}

impl<Type: PartialEq + Clone> PartialEq for Bag<Type>
{
    /// Compare this bag to another for equality of value.
    ///
    /// # Parameter: `other`
    /// The bag to compare to `self`.
    ///
    /// # Postcondition
    /// `x == x` is true
    ///
    /// if `x == y`, then `y == x`
    ///
    /// # Return
    /// True if number of elements in `self` and `other` are the same AND if
    /// the values of all the elements in `self` are the same and in the
    /// same position in `other`.

    fn eq( &self, other: &Bag<Type> ) -> bool
    {
        self.data == other.data
    }

    /// Compare this bag to another object for inequality of value.
    ///
    /// # Parameter: `other`
    /// The bag to compare to `self`.
    ///
    /// # Postcondition
    /// `x == x` is true
    ///
    /// if `x != y`, then `y != x`
    ///
    /// # Return
    /// False if `self == other`.

    fn ne( &self, other: &Bag<Type> ) -> bool
    {
        !( self == other )
    }
}

impl<Type: PartialEq + Clone> ::std::ops::AddAssign for Bag<Type>
{
    /// Add the contents of another bag to this bag.
    ///
    /// # Parameter: `other`
    /// A bag whose contents will be added to `self`.
    ///
    /// # Postcondition
    /// The elements from other have been added to `self`.
    ///
    /// `other` will be unmodified.
    ///
    /// # Caution
    ///   Because of how reserve_exact() works, this method does
    ///   not always work as intended.
    ///
    /// # Panics
    /// If `self.data.len() + other.data.len()` would cause an
    /// unsigned integer overflow.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocating a new array.

    fn add_assign( &mut self, other: Bag<Type> )
    {
        let longer_capacity = self.data.len() + other.data.len();

        if longer_capacity > self.data.capacity()
        {
            self.data.reserve_exact( longer_capacity );
        }

        self.data.extend_from_slice( &other.data[ .. ] );
    }
}

impl<Type: PartialEq + Clone> ::std::ops::Add for Bag<Type>
{
    type Output = Bag<Type>;

    /// Create a new bag that contains all the elements from two other bags.
    ///
    /// # Parameter: `other`
    /// The bag to be added with `self`.
    ///
    /// # Postcondition
    /// `self` and `other` are not altered.
    ///
    /// # Return
    /// A new bag that is the union of `self` and `other`.
    ///
    /// # Panics
    /// If `self.data.len() + other.data.len()` would cause an
    /// unsigned integer overflow.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocating a new array.

    fn add( self, other: Bag<Type> ) -> Bag<Type>
    {
        Bag { data: [ &self.data[ .. ]
                    , &other.data[ .. ] ]
                    .concat() }
    }
}

impl<Type: PartialEq + Clone + Default + ::std::fmt::Debug> ::std::fmt::Debug for Bag<Type>
{
    /// Renders the bag's contents into a human readable form.
    ///
    /// # Precondition
    /// The type in the bag implements the trait: `Debug`.
    ///
    /// # Postcondition
    /// The bag is not altered by this method.
    ///
    /// # Panics
    /// If the iterated write! returns an Err.

    fn fmt( &self, fmt: &mut ::std::fmt::Formatter ) -> ::std::fmt::Result
    {
        write!( fmt, "Bag with {:?} elements: [", self.data.len() )?;

        if self.data.len() == 0
        {
            write!( fmt, " ]" )
        }
        else
        {
            self.data[ ..( self.data.len() - 1 ) ].iter().for_each(
                |value| write!( fmt, " {:?},",value ).unwrap() );
                    // the write returned value is dropped instead of propogated

            write!( fmt, " {:?} ] Capacity: {:?}",
                    self.data.last(),
                    self.data.capacity() )
        }
    }
}

impl<Type: PartialEq + Clone + Default + ::std::hash::Hash> ::std::hash::Hash for Bag<Type>
{
    /// Create a hash value for the bag.
    ///
    /// Used for places that need a hash, like a hashmap.
    ///
    /// # Precondition
    /// The type in the bag implements the trait: `Hash`.
    ///
    /// # Postcondition
    /// The bag is not altered by this method.

    fn hash<HashType: ::std::hash::Hasher>( &self, state: &mut HashType )
    {
        self.data.hash( state );
    }
}
