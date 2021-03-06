// File: bag.rs

//! A Bag as an unsorted, ordered collection of generic types in which
//! the same item may appear multiple times. The bag's capacity can
//! grow as needed and can be reduced.
//!
//! The type used must have the traits `PartialEq`, `Clone`, and `Default`.
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

pub use self::len_trait::len::{ Len, Empty, Clear };
pub use self::len_trait::capacity::{ Capacity, WithCapacity, CapacityMut };
pub use std::default::Default;
pub use std::clone::Clone;
pub use std::cmp::PartialEq;
pub use std::ops::{ AddAssign, Add };
pub use std::fmt::Debug;
pub use std::hash::Hash;
pub use std::iter::IntoIterator;
pub use std::iter::Iterator;

/// A container for inserting and removing given values.
///
/// # Invariant of the Bag struct:
/// 1. The number of elements in the bag is in the instance variable used.
///
/// 2. For an empty bag, we do not care what is stored in any of data;
/// for a non-empty bag, the elements in the bag are stored in `data[ 0 ]`
/// through `data[ used - 1 ]`, and we don't care what's in the
/// rest of data.

pub struct Bag<Type: PartialEq + Clone + Default>
{
   data: Vec<Type>,
   used: usize
}

impl<Type: PartialEq + Clone + Default> Bag<Type>
{
    /// Initialize an empty bag.
    /// 
    /// Implementation matches Default Constructor.
    ///
    /// # Return
    /// A bag that is empty and has a capacity of one.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocating a new array.
    ///
    /// # Examples
    /// ```
    /// # #![allow(unused_mut)]
    /// let mut bag = Bag::new();
    /// ```

    pub fn new() -> Self
    {
        Bag { data: vec![ Type::default(); 1 ], used: 0 }
    }

    /// Initialize an empty bag having a capacity of `initial_capacity`.
    ///
    /// # Note
    /// This method has been deprecated in favor of a trait defined equivalent.
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

    pub fn with_capacity( initial_capacity: usize ) -> Self
    {
        if initial_capacity <= 0
        {
            panic!( "initialCapacity must be > 0" );
        }
        else
        {
            Bag { data: vec![ Type::default(); initial_capacity ], used: 0 }
        }
    }

    /// Make a copy from a source.
    /// 
    /// # Note
    /// This method has been deprecated in favor of clone() and clone_from().
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

    pub fn from_bag( source: &Bag<Type> ) -> Self
    {
        Bag { data: source.data.clone() , used: source.used }
    }

    /// Potentially increase capacity of this bag.
    ///
    /// # Note
    /// This method has been deprecated in favor of reserve( ).
    ///
    /// # Parameter: `new_capacity`
    /// An unsigned integer greater than 0.
    ///
    /// # Precondition
    /// `new_capacity` must be greater than 0.
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
    /// # Panics
    /// `new_capacity` given is not greater than 0.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocating a new array.

    pub fn ensure_capacity( &mut self, mut new_capacity: usize )
    {
        if new_capacity <= 0
        {
            panic!( "new_capacity < 1" );
        }
        else
        {
            if self.data.len() < new_capacity
            {
                new_capacity -= self.data.len();
                self.data.extend( vec![ Type::default(); new_capacity ] );
            }
        }
    }

    /// Return the current capacity of the bag.
    ///
    /// # Note
    /// This method has been deprecated in favor of capacity().
    ///
    /// # Postcondition
    /// This method does not alter state of the bag.
    ///
    /// # Return
    /// An unsigned integer that represents total capacity of this bag.

    pub fn get_capacity( &self ) -> usize
    {
        self.data.len()
    }

    /// Erase all copies of a specified element from this bag if target exists in bag.
    /// 
    /// Implementation matches erase( target ).
    ///
    /// # Parameter: `target`
    /// The element(s) to remove from the bag.
    ///
    /// # Postcondition
    /// If `target` was found in the bag, then all copies of
    /// `target` have been removed and the method returns number of items removed.
    ///
    /// Used will change if one is found, but the capacity will not.
    ///
    /// # Return
    /// An unsigned integer value representing the number of items erased from bag.
    ///
    /// # Examples
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
        let old_used = self.used;
        let mut index = 0;

        // loop will only iterate though the full
        //  range of 0 to this.used, which can vary though the loop
        while index < self.used
        {
            if self.data[ index ] == target
            {
               self.used -= 1;
               self.data[ index ] = self.data[ self.used ].clone();
            }
            else
            {
               index += 1;
            }
        }

        old_used - self.used
    }

    /// Remove one copy of a specified element from this bag.
    /// 
    /// Implementation matches erase_one( target ).
    ///
    /// # Parameter: `target`
    /// The element to remove from the bag.
    ///
    /// # Postcondition
    /// If `target` was found in the bag, then only the first copy of
    /// `target` has been removed and the method returns true. 
    /// Otherwise the bag remains unchanged and the method returns false.
    ///
    /// Used will change if one is found, but the capacity will not.
    ///
    /// # Return
    /// True or false depending on whether `target` exists in the bag.
    ///
    /// # Examples
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
        match self.data[ ..self.used ].iter().position( | value | *value == target )
        {
            None    =>      false,
            Some( index ) =>  
                {
                    self.used -= 1;
                    self.data[ index ] = self.data[ self.used ].clone();
                    true
                }
        }
    }

    /// Add a new element to this bag doubling capacity if needed.
    /// 
    /// Implementation matches insert( entry ).
    ///
    /// # Parameter: `new_item`
    /// The new element that is being inserted.
    ///
    /// # Postcondition
    /// A new copy of the element has been added to this bag.
    ///
    /// Used will increase by one, the capacity only change if needed.
    ///
    /// # Panics
    /// If `self.data.len() * 2` causes an unsigned integer overflow.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocating a new array.
    ///
    /// # Examples
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
        if self.used == self.data.len()
        {
            let extra_capacity = self.data.len();
            self.data.extend( vec![ Type::default(); extra_capacity ] );
        }

        self.data[ self.used ] = new_item;
        self.used += 1;
    }

    /// Determine the number of elements in this bag.
    ///
    /// # Note
    /// This method has been deprecated in favor of len().
    ///
    /// # Postcondition
    /// This method does not alter state of the bag.
    ///
    /// # Return
    /// The number of elements in this bag.

    pub fn size( &self ) -> usize
    {
        self.used
    }

    /// Accessor method to count the number of occurrences of a
    /// particular element in this bag.
    /// 
    /// Implementation matches occurrences( target ).
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
        self.data[ ..self.used ].iter()
                                .filter( | &value | *value == target )
                                .count()
    }

    /// Reduces the capacity of this bag to current size if there is
    /// excess capacity.
    ///
    /// # Note
    /// This method has been deprecated in favor of shrink_to_fit().
    ///
    /// # Postcondition
    /// Capacity of this bag is reduced to the current number
    /// of items in bag or left unchanged if capacity equals to
    /// number of items in bag but must be at least one.

    pub fn trim_to_size( &mut self )
    {
        self.data.truncate(
            if self.used <= 1
            {
                1
            }
            else
            {
                self.used
            }
        );
    }
}

impl<Type: PartialEq + Clone + Default> Len for Bag<Type>
{
    /// Determine the number of elements in this bag.
    /// 
    /// Implementation matches size().
    ///
    /// # Postcondition
    /// This method does not alter state of the bag.
    ///
    /// # Return
    /// The number of elements in this bag.
    ///
    /// # Examples
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
        self.used
    }
}

impl<Type: PartialEq + Clone + Default> Empty for Bag<Type>
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
    /// ```
    /// let mut bag = Bag::new();
    /// assert!( bag.is_empty() );
    ///
    /// bag.insert( 1 );
    /// assert!( !bag.is_empty() );
    /// ```

    fn is_empty( &self ) -> bool
    {
        self.used < 1
    }
}

impl<Type: PartialEq + Clone + Default> Clear for Bag<Type>
{
    /// Empty the bag.
    ///
    /// # Postcondition
    /// The capacity of the bag is not altered.
    ///
    /// # Examples
    /// ```
    /// let mut bag = Bag::new();
    /// bag.insert( 1 );
    /// bag.insert( 2 );
    /// bag.insert( 3 );
    /// assert!( !bag.is_empty() );
    ///
    /// bag.clear();
    /// assert!( bag.is_empty() );
    /// ```

    fn clear(&mut self)
    {
        self.used = 0;
    }
}

impl<Type: PartialEq + Clone + Default> Capacity for Bag<Type>
{
    /// Return the current capacity of the bag.
    /// 
    /// Implementation matches getCapacity().
    ///
    /// # Postcondition
    /// This method does not alter state of the bag.
    ///
    /// # Return
    /// An unsigned integer that represents total capacity of this bag.
    ///
    /// # Examples
    /// ```
    /// let mut bag = Bag::with_capacity( 10 );
    /// assert_eq!( bag.capacity(), 10 );
    /// ```

    fn capacity( &self ) -> usize
    {
        self.data.len()
    }
}

impl<Type: PartialEq + Clone + Default> WithCapacity for Bag<Type>
{
    /// Initialize an empty bag having a capacity of `initial_capacity`.
    /// 
    /// Implementation matches constructor with expressed capacity.
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
            Bag { data: vec![ Type::default(); initial_capacity ], used: 0 }
        }
    }
}

impl<Type: PartialEq + Clone + Default> CapacityMut for Bag<Type>
{
    /// Potentially increase capacity of this bag.
    /// 
    /// Implementation matches ensureCapacity( newCapacity ).
    ///
    /// # Parameter: `new_capacity`
    /// An unsigned integer greater than 0.
    ///
    /// # Precondition
    /// `new_capacity` must be greater than 0.
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
    /// # Panics
    /// `new_capacity` given is not greater than 0.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocating a new array.
    ///
    /// # Examples
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
            if self.data.len() < new_capacity
            {
                new_capacity -= self.data.len();
                self.data.extend( vec![ Type::default(); new_capacity ] );
            }
        }
    }

    /// Reduces the capacity of this bag to current size if there is
    /// excess capacity.
    /// 
    /// Implementation matches trimToSize().
    ///
    /// # Postcondition
    /// Capacity of this bag is reduced to the current number
    /// of items in bag or left unchanged if capacity equals to
    /// number of items in bag but must be at least one.
    ///
    /// # Examples
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
        self.data.truncate(
            if self.used <= 1
            {
                1
            }
            else
            {
                self.used
            }
        );
    }
}

impl<Type: PartialEq + Clone + Default> Default for Bag<Type>
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
        Bag { data: vec![ Type::default(); 1 ], used: 0 }
    }
}

impl<Type: PartialEq + Clone + Default> Clone for Bag<Type>
{
    /// Generate a copy of this bag.
    /// 
    /// Implementation matches copy constructor and assignment operator.
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
        Bag { data: self.data.clone() , used: self.used }
    }

    /// Make a copy from a source.
    /// 
    /// Implementation matches copy constructor.
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
        self.used = source.used;
    }
}

impl<Type: PartialEq + Clone + Default> PartialEq for Bag<Type>
{
    /// Compare this bag to another for equality of value.
    /// 
    /// Implementation matches equality operator.
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
        self.data[ ..self.used ] == other.data[ ..other.used ]
    }

    /// Compare this bag to another object for inequality of value.
    /// 
    /// Implementation matches inequality operator.
    ///
    /// # Parameter: `other`
    /// The  bag to compare to `self`.
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

impl<Type: PartialEq + Clone + Default> AddAssign for Bag<Type>
{
    /// Add the contents of another bag to this bag.
    /// 
    /// Implementation matches add assignment operator.
    ///
    /// # Parameter: `other`
    /// A bag whose contents will be added to `self`.
    ///
    /// # Postcondition
    /// The elements from other have been added to `self`.
    ///
    /// `other` will be unmodified.
    ///
    /// # Panics
    /// If `self.used + other.used` would cause an
    /// unsigned integer overflow.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocating a new array.

    fn add_assign( &mut self, other: Bag<Type> )
    {
        let mut old_capacity = self.data.len();

        self.data.truncate( self.used );
        self.data.extend_from_slice( &other.data[ ..other.used ] );
        self.used += other.used;

        if self.used < old_capacity
        {
            old_capacity -= self.data.len();
            self.data.extend( vec![ Type::default(); old_capacity ] );
        }
    }
}

impl<Type: PartialEq + Clone + Default> Add for Bag<Type>
{
    type Output = Bag<Type>;

    /// Create a new bag that contains all the elements from two other bags.
    /// 
    /// Implementation matches addition operator.
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
    /// If `self.used + other.used` would cause an
    /// unsigned integer overflow.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocating a new array.

    fn add( self, other: Bag<Type> ) -> Bag<Type>
    {
        Bag { data: [ &self.data[ ..self.used ]
                , &other.data[ ..other.used ] ]
                .concat()
            , used: self.used + other.used }
    }
}

impl<Type: PartialEq + Clone + Default + Debug> Debug for Bag<Type>
{
    /// Renders the bag's contents into a human readable form.
    /// 
    /// Implementation matches string buffer operator.
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
        write!( fmt, "Bag with {:?} elements: [", self.used )?;

        if self.used == 0
        {
            write!( fmt, " ]" )
        }
        else
        {
            self.data[ ..( self.used - 1 ) ].iter().for_each(
                |value| write!( fmt, " {:?},",value ).unwrap() );
                    // the write returned value is dropped instead of propogated

            write!( fmt, " {:?} ] Capacity: {:?}",
                    self.data[ self.used - 1 ],
                    self.data.len() )
        }
    }
}

impl<Type: PartialEq + Clone + Default + Hash> Hash for Bag<Type>
{
    /// Create a hash value for the bag.
    /// 
    /// Implementation matches hashCode().
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
        self.used.hash( state );
        self.data[ ..self.used ].hash( state );
    }
}

/// An iterator that references a Bag structure

pub struct BagIterator<'a, Type: 'a + PartialEq + Clone + Default>
{
    bag: &'a Bag<Type>,
    index: Option<usize>
}

impl<'a, Type: PartialEq + Clone + Default> IntoIterator for &'a Bag<Type>
{
    type Item = &'a Type;
    type IntoIter = BagIterator<'a, Type>;

    /// Generate an iterator for the Bag.
    ///
    /// # Postcondition
    /// The Bag is not altered.
    ///
    /// # Return
    /// A new iterator referencing the Bag.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocation.

    fn into_iter( self ) -> Self::IntoIter
    {
        BagIterator { bag: self, index: None }
    }
}

impl<'a, Type: PartialEq + Clone + Default> BagIterator<'a, Type>
{
    /// Generate an iterator for the Bag.
    ///
    /// # Postcondition
    /// The Bag is not altered.
    ///
    /// # Return
    /// A new iterator referencing the Bag.
    ///
    /// # Aborts
    /// OOM: Insufficient memory for allocation.

    pub fn new( source: &'a Bag<Type> ) -> BagIterator<'a, Type>
    {
        BagIterator { bag: source, index: None }
    }
}

impl<'a, Type: PartialEq + Clone + Default> Iterator for BagIterator<'a, Type>
{
    type Item = &'a Type;

    /// Advances the iterator and returns the next value.
    ///
    /// # Postcondition
    /// The bag is not altered by this method and the iterator's index will change.
    ///
    /// # Return
    /// The next value in the bag or None if the iteration is finished.
    ///
    /// # Examples
    /// ```
    /// let mut bag = Bag::new();
    /// bag.insert( 1 );
    /// bag.insert( 2 );
    /// bag.insert( 3 );
    ///
    /// let mut iter = bag.into_iter();
    ///
    /// // A call to next() returns the next value...
    /// assert_eq!( Some( &1 ), iter.next() );
    /// assert_eq!( Some( &2 ), iter.next() );
    /// assert_eq!( Some( &3 ), iter.next() );
    ///
    /// // ... and then None once it's over.
    /// assert_eq!( None, iter.next() );
    ///
    /// // More calls may or may not return None. Here, they always will.
    /// assert_eq!( None, iter.next() );
    /// assert_eq!( None, iter.next() ); 
    /// ```

    fn next( &mut self ) -> Option<Self::Item>
    {
        let next_index = 
            match self.index
            {
                Some( i ) => i + 1,
                None => 0
            };

        self.index = Some( next_index );
        self.bag.data[ ..self.bag.used ].get( next_index )
    }

    /// Consumes the iterator, counting the number of iterations remaining.
    ///
    /// # Postcondition
    /// The bag is not altered by this method and the iterator is consumed.
    ///
    /// # Return
    /// The number of elements left in the iterator.
    ///
    /// # Examples 
    /// ```
    /// let mut bag = Bag::new();
    /// bag.insert( 1 );
    /// bag.insert( 2 );
    /// bag.insert( 3 );
    /// assert_eq!( bag.into_iter().count(), 3 );
    ///
    /// bag.insert( 4 );
    /// bag.insert( 5 );
    /// assert_eq!( bag.into_iter().count(), 5 ); 
    /// ```

    fn count( self ) -> usize
    {
        match self.index
        {
            Some( i ) => {
                            if self.bag.used - i > 0
                            {
                                self.bag.used - i
                            }
                            else
                            {
                                0
                            }
                        },
            None => self.bag.used
        }
    }

    /// Consumes the iterator, returning the last element.
    ///
    /// # Postcondition
    /// The bag is not altered by this method and the iterator is consumed.
    ///
    /// # Return
    /// The last element in the iterator or None if the lenght is zero.
    ///
    /// # Examples 
    /// ```
    /// let mut bag = Bag::new();
    /// bag.insert( 5 );
    /// bag.insert( 4 );
    /// bag.insert( 3 );
    /// assert_eq!( bag.into_iter().last(), Some( &3 ) );
    ///
    /// bag.insert( 2 );
    /// bag.insert( 1 );
    /// assert_eq!( bag.into_iter().last(), Some( &1 ) ); 
    /// ```

    fn last( self ) -> Option<Self::Item>
    {
        if self.bag.used > 0
        {
            self.bag.data.get( self.bag.used - 1 )
        }
        else
        {
            None
        }
    }

    /// Returns the nth element of the iterator.
    ///
    /// # Parameter: `n`
    /// The non-negative offset from the iterator's index to access.
    ///
    /// # Postcondition
    /// The bag is not altered by this method and the iterator's index may change.
    ///
    /// # Return
    /// The nth element of the iterator or None if n is beyond the lenght of the iterator.
    ///
    /// # Note
    /// The nth element of the iterator is not necessarily the nth element in the bag.
    /// 
    /// Calling nth( _ ) will not reset the iterator's index to its original state.
    ///
    /// # Examples
    /// ```
    /// let mut bag = Bag::new();
    /// bag.insert( 1 );
    /// bag.insert( 2 );
    /// bag.insert( 3 );
    ///
    /// let mut iter = bag.into_iter();
    ///
    /// assert_eq!( iter.nth( 1 ), Some( &2 ) );
    /// assert_eq!( iter.nth( 1 ), None ); 
    /// ```

    fn nth( &mut self, n: usize ) -> Option<Self::Item>
    {
        let next_index = 
            match self.index
            {
                Some( i ) => i + n + 1,
                None => n
            };

        self.index = Some( next_index );
        self.bag.data[ ..self.bag.used ].get( next_index )
    }
}
