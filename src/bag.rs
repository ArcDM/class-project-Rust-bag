// File: bag.rs 

import java.util.Arrays;
import java.util.Objects;


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
///    February 10, 2018


#[derive(Copy, Clone, Debug)]
pub struct Bag<Type>
{
   // Invariant of the DoubleArrayBag class:
   //   1. The number of elements in the bag is in the instance variable 
   //      used, which is no more than data.length.
   //   2. For an empty bag, we do not care what is stored in any of data;
   //      for a non-empty bag, the elements in the bag are stored in data[0]
   //      through data[used-1], and we don't care what's in the
   //      rest of data.

   data: Vec<Type>,
   used: u16
}

   /// Initialize an empty bag
   /// post.
   ///   This bag is empty and has a capacity of 1.
   /// exception OutOfMemoryError <?>
   ///   Indicates insufficient memory for allocating the array 

   public DoubleArrayBag( ) {
      this.used = 0;
      this.data = new double[1];
   }


    /// Initialize an empty bag having a capacity of initialCapacity
    /// param:
    ///   initialCapacity an int greater than 0
    /// pre:
    ///   initialCapacity must be greater than 0
    /// post:
    ///   This bag is empty and has a capacity of initialCapacity
    /// exception: <?>
    ///   IllegalArgumentException thrown if initialCapacity not greater than 0
    ///   OutOfMemoryError thrown if insufficient memory for allocating the array

   public DoubleArrayBag( int initialCapacity ) {
       if ( initialCapacity < 1 )
           throw new IllegalArgumentException("initialCapacity < 1");
       this.used = 0;
       try {
           this.data = new double[initialCapacity];
       }
       catch (OutOfMemoryError err) {
           int reducedCapacity = initialCapacity / 2;
           this.data = new double[reducedCapacity];
           throw new OutOfMemoryError("Could not accommodate request -- cut in half");
       }
   }


    /// Initialize a new bag as an exact copy of source
    /// param: source
    ///   reference to bag that is to be copied
    /// exception: <?> NullPointerException
    ///   occurs when source is null
    /// post:
    ///   This bag is a copy of source and has a capacity of
    ///   of number of elements in source

   public DoubleArrayBag( DoubleArrayBag source ) {
       if ( source == null ) {
           throw new NullPointerException("source must not be null");
       }

       this.used = source.used;
       this.data = new double[source.used];
       /* below logic we would write if we did not have System.arraycopy
       for (int index = 0; index < source.used; ++index) {
           this.data[index] = source.data[index];
       }
       *
       * Documentation from Oracle about System.arraycopy is:
        * arraycopy(Object src, int srcPos, Object dest, int destPos, int length)
        * Copies an array from the specified source array, beginning at the specified
        * position, to the specified position of the destination array.
        */
       System.arraycopy(source.data, 0, this.data, 0, source.used);
   }

     

   /// Generate a copy of this bag.
   /// post:
   ///    x.clone() != x
   ///    x.clone().getClass() == x.getClass()
   ///    x.clone().equals( x )
   ///
   /// return:
   ///   The return value is a copy of this bag. Subsequent changes to the
   ///   copy will not affect the original, nor vice versa.

   public DoubleArrayBag clone( ) {  // Clone an DoubleArrayBag object.
      DoubleArrayBag answer;
      
      try {
         answer = (DoubleArrayBag) super.clone( );
         answer.data = data.clone( );
         return answer;
      }
      catch (CloneNotSupportedException e) {  
         // This exception should not occur. But if it does, it would probably
         // indicate a programming error that made super.clone unavailable.
         // The most common error would be forgetting the "Implements Cloneable"
         // clause at the start of this class.
         throw new RuntimeException ("This class does not implement Cloneable");
      }
   }

      

    /// Compare this DoubleArrayBag to another object for equality of value
    /// param: other
    ///   reference to another DoubleArrayBag
    ///
    /// post:
    ///   x.equals(x) is true
    ///   if x.equals(y) then y.equals(x)
    ///   x.equals(null) is false
    ///
    /// return:
    ///   true if number of elements in this and other are the same AND if
    ///   the values of all the elements in the bag are the same and in the
    ///   same position in the bag
    ///
    /// note:
    ///   If the value of other is null, then the return is false

   @Override
   public boolean equals(Object other) {
       if ( other == null ) return false;

       boolean isEqual = false;
       if ( other instanceof DoubleArrayBag ) {
           DoubleArrayBag candidate = (DoubleArrayBag) other;
           //are we comparing this with this
           if (this == other) return true;

           if ( candidate.used != this.used ) return false;
           if ( candidate.hashCode() != this.hashCode() ) return false;

           int index = 0;
           isEqual = true;
           while ( isEqual && index < this.used ) {
               if ( this.data[ index ] != candidate.data[ index ] )
                   isEqual = false;
               else
                   index++;
           }
       }
       return isEqual;
   }


    /// Potentially increase capacity of this bag
    ///
    /// pre:
    ///   newCapacity must be greater than 0
    ///
    /// post:
    ///   The bag's capacity is at least newCapacity.  If the capacity
    ///   was already at or greater than newCapacity, then the capacity
    ///   is left unchanged.
    ///
    /// param:
    ///   newCapacity an integer greater than 0
    ///
    /// exception: <?>
    ///   OutOfMemoryError if not enough dynamic available to allocate
    ///   additional space
    /// exception: <?>
    ///   IllegalArgumentException if newCapacity not greater than 0

   public void ensureCapacity( int newCapacity ) {
       if ( newCapacity < 1)
           throw new IllegalArgumentException("newCapacity < 1");

       if ( this.data.length < newCapacity ) {
           double[] biggerArray = new double[newCapacity];
           System.arraycopy(this.data, 0, biggerArray, 0, this.used);
           this.data = null;
           this.data = biggerArray;
       }
   }


    /// returns the current capacity of the bag
    /// post:
    ///   This method does not alter state of the bag
    /// return:
    ///   an int that represents total capacity of this bag

   public int getCapacity( ) {
       return this.data.length;
   }

   @Override 
   public int hashCode( ) {
       // fill all values with 0 in this.data from used through length - 1
       if ( this.used < this.data.length) 
          Arrays.fill( this.data, this.used, this.data.length - 1, 0);

       return Objects.hash( this.used ) + Arrays.hashCode( this.data );
   }


   /// Erases all copies of a specified element from this bag if target exists in bag.
   /// param: target
   ///   the element(s) to remove from the bag
   /// post:
   ///   If target was found in the bag, then all copies of
   ///   target have been removed and the method returns number of items removed. 
   /// return:
   ///   int value from 0 to number of items erased from bag

   public int erase( int target ) {
       int count = 0;
       int index = 0;

       while (index < this.used) {
           if (this.data[index] == target) {
               --this.used;
               this.data[index] = this.data[this.used];
               ++count;
           }
           else
               ++index;
       }

       return count;
   }
              

   /// Remove one copy of a specified element from this bag.
   /// param: target
   ///   the element to remove from the bag
   /// post:
   ///   If target was found in the bag, then one copy of
   ///   target has been removed and the method returns true. 
   ///   Otherwise the bag remains unchanged and the method returns false. 
   /// return:
   ///   true or false depending on whether target exists in the bag

   public boolean erase_one(int target) {
      int index; // The location of target in the data array.
       
      // First, set index to the location of target in the data array,
      // which could be as small as 0 or as large as used-1; If target
      // is not in the array, then index will be set equal to used;
      for (index = 0; (index < used) && (target != data[index]); index++) {
         // No work is needed in the body of this for-loop.
      }   
         
      if (index == used)
         // The target was not found, so nothing is removed.
         return false;
      else {
         // The target was found at data[index].
         // So reduce used by 1 and copy the last element onto data[index].
         used--;
         data[index] = data[used];
         return true;
      }
   }
   

   /// Add a new element to this bag doubling capacity if needed
   /// 
   /// param: newItem
   ///   the new element that is being inserted
   /// post:
   ///   A new copy of the element has been added to this bag.

   public void insert(double newItem) {
      if ( used == this.data.length ) {
          this.ensureCapacity( this.data.length /// 2 );
      }
      data[ used++ ] = newItem;
   }


   /// Add the contents of another bag to this bag.
   /// param: addend
   ///   a bag whose contents will be added to this bag
   /// pre:
   ///   The parameter, addend, is not null. 
   /// post:
   ///   The elements from addend have been added to this bag.
   /// exception: <?> NullPointerException
   ///   Indicates that addend is null. 
   /// exception: <?> OutOfMemoryError
   ///   Indicates insufficient memory to add addend to this bag

   public void plusEquals(DoubleArrayBag addend) {
       if ( addend == null ) {
           throw new NullPointerException("Cannot add from null object");
       }
       if (this.used + addend.used > this.data.length ) {
           this.ensureCapacity( this.used + addend.used );
       }
         
      System.arraycopy(addend.data, 0, this.data, this.used, addend.used);
      used += addend.used;
   }   


   /// Determine the number of elements in this bag.
   /// return:
   ///   the number of elements in this bag

   public int size( ) {
      return used;
   }
   

   /// Accessor method to count the number of occurrences of a particular element
   /// in this bag.
   /// param: target
   ///   the element for which number of occurrences will be counted 
   /// return:
   ///   the number of times that target occurs in this bag

   public int occurrences(int target) {
      int answer = 0;
      
      for (int index = 0; index < used; index++) {
         if (target == data[index]) answer++; 
      }
      return answer;
   }


    /// This method renders the bag's contents into a human readable form
    /// post:
    ///    The bag is not altered by this method

   public String toString() {
       // StringBuffer is used since its contents are mutable.  Strings are
       // immutable and, thus, very inefficient for such purpose
       StringBuffer sb = new StringBuffer();
       sb.append("DoubleArrayBag with ");
       sb.append(this.size());
       sb.append(" elements: [");
       if ( this.size() == 0 ) {
           sb.append(" ]");
           return sb.toString();
       }
       for (int i=0; i < this.size()-1; ++i) {
           sb.append(" ");
           sb.append(this.data[i]);
           sb.append(",");
       }
       sb.append(" "); sb.append(this.data[this.size()-1]);
       sb.append(" ]");
       sb.append(" Capacity: "); sb.append(this.getCapacity());
       return sb.toString();
   }


    /// reduce capacity of this bag to current size if there is
    /// excess capacity
    ///
    /// post:
    ///   capacity of this bag is reduced to the current number
    ///   of items in bag or left unchanged if capacity equals to
    ///   number of items in bag but must be at least 1
    ///
    /// exception:
    ///   OutOfMemoryError if not enough dynamic memory to
    ///   support allocation and deallocation of memory

    public void trimToSize( ) {
        if ( this.used < this.data.length ) {
            int newCapacity;
            if ( this.used <= 1 )
                newCapacity = 1;
            else
                newCapacity = this.used;

            double[] trimmedArray = new double[ newCapacity ];
            System.arraycopy( trimmedArray, 0, this.data, 0, this.used );
            this.data = null;
            this.data = trimmedArray;
            trimmedArray = null;
        }
    }


   /// Create a new bag that contains all the elements from two other bags -- note that
   /// this is a class method NOT an instance method and must be called with class
   /// name qualifier.
   /// param: b1
   ///   the first of two bags
   /// param: b2
   ///   the second of two bags
   /// pre:
   ///   Neither b1 nor b2 is null, and b1.size() + b2.size() cannot
   ///   exceed MAX_CAPACITY
   /// post:
   ///   bag referenced by b1 and bag reference by b2 are not altered
   /// return:
   ///   the union of b1 and b2
   /// exception: <?> NullPointerException
   ///   Indicates that one of the arguments is null.
   /// exception: <?> OutOfMemoryError
   ///   Indicates insufficient memory for the new bag.

   public static DoubleArrayBag union(DoubleArrayBag b1, DoubleArrayBag b2) {
      if ( b1 == null || b2 == null ) {
          throw new NullPointerException("one or both bags reference is null");
      }

      DoubleArrayBag newBag = new DoubleArrayBag( b1.size() + b2.size() );
      
      /* below code necessary if we didn't have plusEquals method
      System.arraycopy(b1.data, 0, newBag.data, 0, b1.size() );
      System.arraycopy(b2.data, 0, newBag.data, b1.size(), b2.size());
      newBag.used = b1.size() + b2.size();
      */
      newBag.plusEquals(b1);
      newBag.plusEquals(b2);

      return newBag;
   }

}
