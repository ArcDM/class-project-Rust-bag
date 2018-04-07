// File: bagf64.rs 

#![ allow( dead_code ) ]

/// An Bag as an unsorted, ordered collection of f64s and in which
/// the same number may appear multiple times. The bag's capacity can
/// grow as needed and can be reduced.
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
///    7.April.2018

pub struct Bag
{
   /// Invariant of the Bag struct:
   ///   1. The number of elements in the bag is in the instance variable used.
   ///   2. For an empty bag, we do not care what is stored in any of data;
   ///      for a non-empty bag, the elements in the bag are stored in data[ 0 ]
   ///      through data[ used - 1 ], and we don't care what's in the
   ///      rest of data.

   data: Vec<f64>,
   used: usize
}

impl Bag
{
    /// Initialize an empty bag.
    /// Return:
    ///     A bag that is empty and has a capacity of 1.
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array.

    pub fn new() -> Self
    {
        Bag { data: vec![ f64::default(); 1 ], used: 0 }
    }

    /// Initialize an empty bag having a capacity of initialCapacity.
    /// Parameter: initial_capacity
    ///     An unsigned integer greater than 0.
    /// Precondition:
    ///     initial_capacity must be greater than 0.
    /// Return:
    ///     A bag that is empty and has a capacity of initial_capacity.
    /// # Panics
    ///     InitialCapacity given is not greater than 0.
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array.

    pub fn with_capacity( initial_capacity: usize ) -> Self
    {
        if initial_capacity <= 0
        {
            panic!( "initialCapacity must be > 0" );
        }
        else
        {
            Bag { data: vec![ f64::default(); initial_capacity ], used: 0 }
        }
    }

    /// Initialize a new bag as an exact copy of source.
    /// Depreciated with the addition of Clone()
    /// Parameter: source
    ///     A reference to a bag that is to be copied.
    /// Postcondition:
    ///     This bag is a copy of source and has a capacity of
    ///     of number of elements in source. Subsequent changes to
    ///     the copy will not affect the original, nor vice versa.
    /// Return:
    ///     A bag that is equivalent to the bag supplied.
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array.

    pub fn from_bag( source: &Bag ) -> Self
    {
        Bag { data: source.data.clone() , used: source.used }
    }
    
    /// Potentially increase capacity of this bag.
    /// Parameter: new_capacity
    ///     An unsigned integer greater than 0.
    /// Precondition:
    ///     new_capacity must be greater than 0.
    /// Postcondition:
    ///     The bag's capacity is at least newCapacity.  If the capacity
    ///     was already at or greater than newCapacity, then the capacity
    ///     is left unchanged.
    /// # Panics
    ///     InitialCapacity given is not greater than 0.
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array.

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
                self.data.extend( vec![ f64::default(); new_capacity ] );
            }
        }
    }

    /// Return the current capacity of the bag.
    /// Postcondition:
    ///     This method does not alter state of the bag.
    /// Return:
    ///     An unsigned integer that represents total capacity of this bag.

    pub fn get_capacity( &self ) -> usize
    {
        self.data.len()
    }

    /// Erase all copies of a specified element from this bag if target exists in bag.
    /// Parameter: target
    ///     the element(s) to remove from the bag.
    /// Postcondition:
    ///     If target was found in the bag, then all copies of
    ///     target have been removed and the method returns number of items removed.
    ///     Used will change if one is found, but the capacity will not.
    /// Return:
    ///     An unsigned integer value representing the number of items erased from bag.

    pub fn erase( &mut self, target: &f64 ) -> usize
    {
        let old_used = self.used;
        let mut index = 0;

        // loop will only iterate though the full
        //  range of 0 to this.used, which can vary though the loop
        while index < self.used
        {
            if self.data[ index ] == *target
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
    /// Parameter: target
    ///     The element to remove from the bag.
    /// Postcondition:
    ///     If target was found in the bag, then one copy of
    ///     target has been removed and the method returns true. 
    ///     Otherwise the bag remains unchanged and the method returns false.
    ///     Used will change if one is found, but the capacity will not.
    /// Return:
    ///     True or false depending on whether target exists in the bag.

    pub fn erase_one( &mut self, target: &f64 ) -> bool
    {
        match self.data[ ..self.used ].iter().position( | value | value == target )
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
    /// Parameter: new_item
    ///     The new element that is being inserted.
    /// Postcondition:
    ///     A new copy of the element has been added to this bag.
    ///     Used will increase by one, the capacity only change if needed.
    /// # Panics
    ///     If "self.data.len() * 2" causes an unsigned integer overflow.
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array.

    pub fn insert( &mut self, new_item: &f64 )
    {
        if self.used == self.data.len()
        {
            let new_capacity = self.data.len() * 2 as usize;
            self.ensure_capacity( new_capacity );
        }

        self.data[ self.used ] = ( *new_item ).clone();
        self.used += 1;
    }

    /// Determine the number of elements in this bag.
    /// Postcondition:
    ///     This method does not alter state of the bag.
    /// Return:
    ///     The number of elements in this bag.

    pub fn size( &self ) -> usize
    {
        self.used
    }   

    /// Accessor method to count the number of occurrences of a
    /// particular element in this bag.
    /// Parameter: target
    ///     The element for which number of occurrences will be counted.
    /// Postcondition:
    ///     This method does not alter state of the bag.
    /// Return:
    ///     The number of times that target occurs in this bag.

    pub fn occurrences( &self, target: &f64 ) -> usize
    {
        self.data[ ..self.used ].iter()
                                .filter( | &value | value == target )
                                .count()
    }

    /// Reduces the capacity of this bag to current size if there is
    /// excess capacity.
    /// Postcondition:
    ///   Capacity of this bag is reduced to the current number
    ///   of items in bag or left unchanged if capacity equals to
    ///   number of items in bag but must be at least one.

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

impl Default for Bag
{
    /// Initialize an empty bag.
    /// Return:
    ///     A bag that is empty and has a capacity of 1.
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array.

    fn default() -> Self
    {
        Bag { data: vec![ f64::default(); 1 ], used: 0 }
    }
}

impl Clone for Bag
{
    /// Generate a copy of this bag.
    /// Postcondition:
    ///     The clone is not the same object as the sourse.
    ///     The clone is the same type as the sourse.
    ///     The clone will equal the sourse as long as both
    ///     bags are unaltered.
    /// Return:
    ///     A new bag initialized as bag with all the elements in
    ///     source and with capacity to equal that number of elements.
    ///     Subsequent changes to the copy will not affect the original,
    ///     nor vice versa.
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array.

    fn clone( &self ) -> Self
    {
        Bag { data: self.data.clone() , used: self.used }
    }

    /// Make a copy from a source.
    /// Parameter: source
    ///     A reference to a bag that is to be copied.
    /// Postcondition:
    ///     This bag is now a copy of source and has a capacity of
    ///     of number of elements in source. Subsequent changes to
    ///     the copy will not affect the original, nor vice versa.
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array.

    fn clone_from( &mut self, source: &Self )
    {
        self.data = source.data.clone();
        self.used = source.used;
    }
}

impl ::std::cmp::PartialEq for Bag
{
    /// Compare this DoubleArrayBag to another object for equality of value.
    /// Parameter: self
    ///     The first of two bags.
    /// Parameter: other
    ///     The second of two bags.
    /// Postcondition:
    ///     x == x is true
    ///     if x == y, then y == x
    /// Return:
    ///     True if number of elements in self and other are the same AND if
    ///     the values of all the elements in self are the same and in the
    ///     same position in other.

    fn eq( &self, other: &Bag ) -> bool
    {
        self.data[ ..self.used ] == other.data[ ..other.used ]
    }

    /// Compare this DoubleArrayBag to another object for inequality of value.
    /// Parameter: self
    ///     The first of two bags.
    /// Parameter: other
    ///     The second of two bags.
    /// Postcondition:
    ///     x == x is true
    ///     if x != y, then y != x
    /// Return:
    ///     False if self == other.

    fn ne( &self, other: &Bag ) -> bool
    {
        !( self == other )
    }
}

impl ::std::ops::AddAssign for Bag
{
    /// Add the contents of another bag to this bag.
    /// Parameter: self
    ///     The bag that will be added to.
    /// Parameter: other
    ///     A bag whose contents will be added to self.
    /// Postcondition:
    ///     The elements from other have been added to self.
    ///     Other will be unmodified.
    /// # Panics
    ///     If "self.used + other.used" would cause an
    ///     unsigned integer overflow.
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array.

    fn add_assign( &mut self, other: Bag )
    {
        let mut old_capacity = self.data.len();

        self.data.truncate( self.used );
        self.data.extend_from_slice( &other.data[ ..other.used ] );
        self.used += other.used;

        if self.used < old_capacity
        {
            old_capacity -= self.data.len();
            self.data.extend( vec![ f64::default(); old_capacity ] );
        }
    }
}

impl ::std::ops::Add for Bag
{
    /// Create a new bag that contains all the elements from two other bags.
    /// Parameter: self
    ///     The first of two bags.
    /// Parameter: other
    ///     The second of two bags.
    /// Postcondition:
    ///     The bag referenced by b1 and bag reference by b2 are not altered.
    /// Return:
    ///     A new bag that is the union of b1 and b2.
    /// # Panics
    ///     If "self.used + other.used" would cause an
    ///     unsigned integer overflow.
    /// # Aborts
    ///     OOM: Insufficient memory for allocating a new array.

    type Output = Bag;

    fn add( self, other: Bag ) -> Bag
    {
        Bag { data: [ &self.data[ ..self.used ]
                , &other.data[ ..other.used ] ]
                .concat()
            , used: self.used + other.used }
    }
}

impl ::std::fmt::Debug for Bag
{
    /// Renders the bag's contents into a human readable form.
    /// Precondition:
    ///     The type in the bag implements the trait: Debug.
    /// Postcondition:
    ///     The bag is not altered by this method.
    /// # Panics
    ///     If the iterated write! returns an Err.

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
