// File: bag.rs 

use std;
use std::cmp::PartialEq;

/// An DoubleArrayBag is an unordered collection of double numbers and in which
/// the same number may appear multiple times.  The bag's capacity can grow as
/// needed and can be reduced.
///
/// note:
///   Because of the slow linear algorithms of this
///   class, large bags will have poor performance.
///
/// author:
///    Nathan Bradley based on the java implementation from
///    H. Paul Haiduk with credit given to Michael Main
///
/// version:
///    2.March.2018

// #[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bag<Type: PartialEq + Clone>
{
   // Invariant of the DoubleArrayBag class:
   //   1. The number of elements in the bag is in the instance variable 
   //      used, which is no more than data.length.
   //   2. For an empty bag, we do not care what is stored in any of data;
   //      for a non-empty bag, the elements in the bag are stored in data[0]
   //      through data[used-1], and we don't care what's in the
   //      rest of data.

   data: Vec<Type>,
   used: usize
}

impl<Type: PartialEq + Clone> Bag<Type>
{
    /// Initialize an empty bag
    /// Return:
    ///     A bag is empty and has a capacity of 0
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array

    pub fn new() -> Self
    {
        Bag { data: Vec::with_capacity( 1 ), used: 0 }
    }

    /// Initialize an empty bag having a capacity of initialCapacity
    /// Parameter: initial_capacity
    ///     An unsigned integer greater than 0
    /// Precondition:
    ///     initial_capacity must be greater than 0
    /// Return:
    ///     A bag is empty and has a capacity of initial_capacity
    /// # Panics
    ///     InitialCapacity given is not greater than 0
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array

    pub fn with_capacity( initial_capacity: usize ) -> Self
    {
        if initial_capacity <= 0
        {
            panic!( "initialCapacity must be > 0" );
        }
        else
        {
            Bag { data: Vec::with_capacity( initial_capacity ), used: 0 }
        }
    }
    
    /// Potentially increase capacity of this bag
    /// Parameter: new_capacity
    ///     An unsigned integer greater than 0
    /// Precondition:
    ///     new_capacity must be greater than 0
    /// Postcondition:
    ///     The bag's capacity is at least newCapacity.  If the capacity
    ///     was already at or greater than newCapacity, then the capacity
    ///     is left unchanged.
    /// # Panics
    ///     InitialCapacity given is not greater than 0
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array

    pub fn ensure_capacity( &mut self, new_capacity: usize )
    {
        if new_capacity <= 0
        {
            panic!( "new_capacity < 1" );
        }
        else
        {
            //self.data.reserve( new_capacity );

            if self.data.len() < new_capacity
            {
                let mut new_data = Vec::with_capacity( new_capacity );
                new_data.extend_from_slice( &self.data[ ..self.used ] );
                self.data = new_data;
            }
        }
    }

    /// Returns the current capacity of the bag
    /// Postcondition:
    ///     This method does not alter state of the bag
    /// Return:
    ///     An unsigned integer that represents total capacity of this bag

    pub fn get_capacity( &self ) -> usize
    {
        self.data.len()
    }

    /// Erases all copies of a specified element from this bag if target exists in bag.
    /// Parameter: target
    ///     the element(s) to remove from the bag
    /// Postcondition:
    ///     If target was found in the bag, then all copies of
    ///     target have been removed and the method returns number of items removed. 
    /// Return:
    ///     An unsigned integer value representing the number of items erased from bag

    pub fn erase( &mut self, target: Type ) -> usize
    {
        let mut remove_count = 0;
        let mut index = 0;

        // loop will only iterate though the full
        //  range of 0 to this.used
        while index < self.used
        {
            if self.data[ index ] == target
            {
               self.used -= 1;
               self.data[ index ] = self.data[ self.used ].clone();
               remove_count += 1;
            }
            else
            {
               index += 1;
            }
        }

        remove_count
    }

    /// Remove one copy of a specified element from this bag.
    /// Parameter: target
    ///     the element to remove from the bag
    /// Postcondition:
    ///     If target was found in the bag, then one copy of
    ///     target has been removed and the method returns true. 
    ///     Otherwise the bag remains unchanged and the method returns false. 
    /// Return:
    ///     true or false depending on whether target exists in the bag

    pub fn erase_one( &mut self, target: Type ) -> bool
    {
        // loop will iterate though the range of 0 to self.used
        //  or exit as a return if self.data[index] == target
        for index in 0..self.used
        {
            if self.data[ index ] == target
            {
               self.used -= 1;
               self.data[ index ] = self.data[ self.used ].clone();
               return true;
            }
        }

        false
    }

    /// Add a new element to this bag doubling capacity if needed
    /// Parameter: new_item
    ///     The new element that is being inserted
    /// Postcondition:
    ///     A new copy of the element has been added to this bag.
    /// # Panics
    ///     "self.data.len() * 2" causes an unsigned integer overflow
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array

    pub fn insert( &mut self, new_item: Type )
    {
        if self.used == self.data.len()
        {
            let new_capacity = self.data.len() * 2 as usize;
            self.ensure_capacity( new_capacity );
        }


        self.data[ self.used ] = new_item;
        self.used += 1;
    }

    /// Determine the number of elements in this bag.
    /// Postcondition:
    ///     This method does not alter state of the bag
    /// Return:
    ///     The number of elements in this bag

    pub fn size( &self ) -> usize
    {
        self.used
    }   

    /// Accessor method to count the number of occurrences of a
    /// particular element in this bag.
    /// Parameter: target
    ///     The element for which number of occurrences will be counted 
    /// Postcondition:
    ///     This method does not alter state of the bag
    /// Return:
    ///     The number of times that target occurs in this bag

    pub fn occurrences( &self, target: Type ) -> usize
    {
        let mut return_count = 0;
      
        for index in 0..self.used
        {
            if target == self.data[ index ]
            {
                return_count += 1;
            }
        }

        return_count
    }

    /// Reduces the capacity of this bag to current size if there is
    /// excess capacity
    /// Postcondition:
    ///   capacity of this bag is reduced to the current number
    ///   of items in bag or left unchanged if capacity equals to
    ///   number of items in bag but must be at least 1
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array

    pub fn trim_to_size( &mut self )
    {
        if self.used < self.data.len()
        {
            let new_capacity;

            if self.used <= 1
            {
                new_capacity = 1;
            }
            else
            {
                new_capacity = self.used;
            }

            //self.data.reserve( new_capacity );

            if self.data.len() < new_capacity
            {
                let mut new_data = Vec::with_capacity( new_capacity );
                new_data.extend_from_slice( &self.data[ ..self.used ] );
                self.data = new_data;
            }
        }
    }
}

impl<Type: PartialEq + Clone> Clone for Bag<Type>
{
    /// Initialize a new bag as an exact copy of source
    /// Postcondition:
    ///     The clone is not the same object as the sourse.
    ///     The clone is the same type as the sourse.
    ///     The clone will equal the sourse as long as both
    ///     bags are unaltered
    /// Return:
    ///     A new bag initialized as bag with all the elements in
    ///     source and with capacity to equal that number of elements.
    ///     Subsequent changes to the copy will not affect the original,
    ///     nor vice versa.
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array

    fn clone( &self ) -> Self
    {
        Bag { data: self.data.clone() , used: self.used }
    }
}

impl<Type: PartialEq + Clone> PartialEq for Bag<Type>
{
    /// Compare this DoubleArrayBag to another object for equality of value
    /// Parameter: self
    ///     The first of two bags
    /// Parameter: other
    ///     The second of two bags
    /// post:
    ///     x == x is true
    ///     if x == y, then y == x
    /// Return:
    ///     true if number of elements in this and other are the same AND if
    ///     the values of all the elements in self are the same and in the
    ///     same position in other

    fn eq( &self, other: &Bag<Type> ) -> bool
    {
        if self.used != other.used
        {
            return false;
        }

        // loop will iterate though the range of 0 to self.used
        //  or exit as a return if self.data[index] != other.data[index]
        for index in 0..self.used
        {
            if self.data[index] != other.data[index]
            {
                return false;
            }
        }

        true
    }

    fn ne( &self, other: &Bag<Type> ) -> bool
    {
        !( self == other )
    }
}

impl<Type: PartialEq + Clone> std::ops::AddAssign for Bag<Type>
{
    /// Add the contents of another bag to this bag.
    /// Parameter: self
    ///     The bag that will be added to
    /// Parameter: other
    ///     A bag whose contents will be added to self
    /// Postcondition:
    ///     The elements from addend have been added to self
    ///     Other will be unmodified
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array

    fn add_assign( &mut self, other: Bag<Type> )
    {
        self.used += other.used;

        if self.used > self.get_capacity()
        {
            let new_capacity = self.used;
            self.ensure_capacity( new_capacity );
        }

        self.data.extend_from_slice( &other.data[ ..other.used ] );
    }
}

impl<Type: PartialEq + Clone> std::ops::Add for Bag<Type>
{
    /// Create a new bag that contains all the elements from two other bags
    /// Parameter: self
    ///     The first of two bags
    /// Parameter: other
    ///     The second of two bags
    /// Postcondition:
    ///     The bag referenced by b1 and bag reference by b2 are not altered
    /// Return:
    ///     A bag that is the union of b1 and b2
    /// # Panics
    ///     self.used + other.used causes an unsigned integer overflow
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array

    type Output = Bag<Type>;

    fn add( self, other: Bag<Type> ) -> Bag<Type>
    {
        let mut return_bag = Bag::with_capacity( self.used + other.used as usize);
        return_bag += self;
        return_bag += other;
        return_bag
    }
}

impl<Type: PartialEq + Clone + std::fmt::Display> std::fmt::Display for Bag<Type>
{
    /// This method renders the bag's contents into a human readable form
    /// Precondition:
    ///     The type in the bag implements the type: Display
    /// Postcondition:
    ///     The bag is not altered by this method

    fn fmt( &self, fmt: &mut std::fmt::Formatter ) -> std::fmt::Result
    {
        write!( fmt, "Bag with {} elements: [", self.used )?;

        if self.used == 0
        {
            write!( fmt, " ]" )
        }
        else
        {
            for index in 0..( self.used - 1 )
            {
                write!( fmt, " {},", self.data[ index ] )?;
            }

            write!( fmt, " {} ] Capacity: {}",
                    self.data[ self.used - 1 ],
                    self.data.len() )
        }
    }
}
