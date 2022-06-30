/*                What You Must Know About Rust

I've written a bunch of Rust programs in order to learn it, but I've
distilled the most important points in the following program, which
I've tried to make as clear and simple as possible.  I expect you to
study this program and understand why the commented out code would
produce errors.  Go through each line in main and study the comments.
But the best way to understand this is to experiment with it.  Get all
resources from rust-lang.org.  This program was compiled with rustc
version 1.39.0.  Earliear versions of rustc may give an error because
they didn't recognize situations where a variable is never used again
after a certain point in the program (putting it in a "cargo" package may
help).  Do rustc wymk.rs to compile and ./wymk (add .exe on pc) to run.

Rust's ability to offer memory safety without a garbage collector rests
on principles called OWNERSHIP, BORROWING and LIFETIME.

Data types in Rust can be divided into two categories: those that can
be copied and those that can't be.  Specifically, a data type can be
copied if it implements the 'Copy' trait (a trait is similar to an
interface).  A type can implement this trait only if it is of fixed
size and is contiguous in memory (and therefore can be completely
allocated on the stack).  Data that are copied include primitive types
for numerical values (i32, u32, f64, etc), booleans, and string
literals or "slices" (rust has different types of strings, some cannot
be copied).  If you define a struct (or enum) that contains only
:Copy-able values, then that struct can also be copied (you can
implement, and often automatically derive the Copy trait for it).  For
example, Rust arrays must have their size known at compile time, and
so an array of :Copy values can itself be copied.  However, if some
part of your data structure contains pointers to heap-allocated data,
such as in the case of a linked list or tree, then it cannot be
copied.  In such a case, you must understand the Rust rules of
OWNERSHIP: each such data structure is 'owned' by a name (a variable).
When this variable goes out of scope, the heap-allocated data is
deallocated.  Rust checks that the heap data is always deallocated and
deallocated exactly once.  The LIFETIME of a variable is at most the
static scope in which it exists, but it can also end when the data it
owns is "moved" to another owner: this happens during variable
assignment, as well as when passing the variable to a function, and
returning one from a function.  During such an operation, if a data
type is :Copy, then a copy will be produced, otherwise it is "moved".
**You cannot use a value once it's moved.**

You can avoid transfers of ownership by creating BORROWS of the
values.  You might think that these are just pointers or references,
but their compile-time meaning is much more than that.  The lifetime
of a borrow cannot be greater than that of the value that it borrows
from (regardless of whether the value can be copied).  An immediate
consequence of this restriction is that you can't return a borrow
(pointer) to a local variable of a function, which is something a lot
of new C programmers will try to do.  In this program, I use fixed
arrays of i32 (signed) integers, which are :Copy, and heap-allocated
vectors of i32's, which are not :Copy to illustrate these essential
concepts of Rust.

Please proceed to 'main' for details.

ALL THE COMMENTED OUT LINES OF CODE WOULD CAUSE COMPILER ERRORS.
*/

// Compiler directives to avoid certain warnings (comment out to experiment):
// we'll have a lot of these because of commented-out code:
#![allow(unused_variables)]
#![allow(unused_mut)]  
#![allow(unused_assignments)]

fn main()    ////////// main
{
  let mut fixeda = [2,4,6]; // A statically fixed-sized array of integers...
  let mut fixedb = fixeda;  // can be copied (:Copy trait)
  fixeda[0] = 9;      // fixeda can still be used because it has its own copy
  println!("fixedb[0] is still {}",fixedb[0]);  // 2, as you'd expect.

  //// If all you have are :Copy types, then Rust can mostly look like any 
  //// other language (but the rules concerning borrowing still apply).
  //   If something does not :Copy, that's when it gets interesting...

  let mut vct1 = vec![1,2,3]; //a vector is heap-allocated, size can change.
  let bvct1 = &vct1; // create borrow
  vct1 = vec![2,4,6];  // previous vector [1,2,3] deallocated when not owned
  //println!("{}",bvct1[0]); // borrow invalidated (compiler error)
  vct1.push(8); // adds new element, expand size of vector
  let mut vct2 = vct1;  // a "transfer of ownership" occurs, NOT A COPY.
  //let mut vct3 = vct1;  // COMPILER ERROR: "value used after move" 

  // We can't use vct1 at all after it's been transferred ("moved") to a new
  // owner, because otherwise, when both vct2 and vct1 go out of scope, the
  // same data will get deallocated twice (seg fault in C).  The lifetime of
  // vct1 terminates here.  However, the variable is still declared and you
  // can bind it to something else:
  vct1 = vec![8,9,10];  // move something back into vct1

  // Ownership is also transferred when you pass a value to a function,
  let mut vct3 = f(vct2);   //... and when a value is returned
  //println!("Do I still have access to vct2? {}",vct2[0]); // NO!

  // So how can I do anything with a variable if gets "moved" so easily?
  // I can BORROW it with a reference (alias), without taking ownership:

  let rv3 = &vct3;  // equivalent to let ref rv3 = vct3; rv3 is a borrow.
  println!("{} and {}",rv3[4], vct3[4]); // vct3 is still alive, and
  for x in &vct3 { println!("{} ",x); }  // can be borrowed more than once
  // Actually when you write vct3[4], you're implicitly also creating a 
  // borrow (same as &vct3[4]): think of vct3[4] as vct->[4] or (*vct).[4] 

  //// But the lifetime of a borrowed reference does not outlive its lender:
  let mut vct4 = vct3;  // data "moved out" of vct3
  //println!("Can I cheat and still use the borrow? {}",rv3[4]); // NO WAY!

  ///// Here is another thing, which caused me some headache at first when
  //    learning rust: when you have a borrow of a non :Copy value, don't
  //    to try to dereference it as it would try to create a temporary copy
  //    of the value, which of course it could not:
  let b4 = &vct4;
  //let vct5 = *b4; // "move occurs ... does not implement the Copy trait"
 

  /////////////  Mutable and Immutable Borrows:
  
  ///// There are also two categories of borrows: mutable and immutable,
  //   the ones above were all immutable.  A mutable borrow can both 
  //   access and change a mutable structure. YOU CANNOT MIX A MUTABLE
  //   BORROW WITH OTHER BORROWS (immutable or mutable) of the same value.
  //   Their lifetimes cannot intersect. Why?  The simple answer is that
  //   the mutable ref invalidates the immutable status of the immutable
  //   borrows. In the case of vectors, destructive operations on a vector
  //   can cause the underlying memory allocated to the vector  to change 
  //   (it could be copied to somewhere else in memory due to resizing).

  // But let's see if I can create some more specific examples:

  let mbv = &mut vct4; // creates an mutable borrow of vct4,
  mbv[1] += 1;         // with which you can mutate the borrowed structure.

  //println!("{} and {} ",&vct4[4], mbv.pop().unwrap()); // NOT OK

  // The line above uses an immutable borrow &vct4 to access the last
  // element of the vector (at index 4) while the mutable borrow tries 
  // to pop the last element off of the vector - do you know what's going
  // to happen?  This is called a "data race". Rust prevents them by
  // not compiling such code.  In Python, with v=[1,3,5,7,9], if you
  // print(v[4],v.pop()) you see 9 9.  But if you print(v.pop(),v[4])
  // you get an index out of range error.  You might say that it's ok as
  // long as the language clearly defines the order in which arguments to
  // a function are evaluated.  But I have another example:

  //   def reverse(A,B):
  //      if len(A) != len(B): return
  //      for i in range(0,len(B)): B[i] = A[len(A)-1-i]

  // Can you determine, looking at just the static code, what's the effect
  // of calling the above function on a pair of lists in python?

  // I'll bet some of you got it wrong.  Consider:
  // V = [1,2,3,4,5]
  // reverse(V,V)   # What's V now?  It's not [5,4,3,2,1] but [5,4,3,4,5]

  // It's not just that such situations might cause confusion for the
  // programmer, but not being able to determine the behavior of a function
  // statically may cost us opportunities to optimize the function (either
  // by ourselfs or by the compiler's static analysis capabilities).
  // Rust would not compile the equivalent call to reverse(V,V) as it
  // would mean passing a mutable reference along with another reference
  // to the same function.  The Rust version of reverse (below) can ONLY
  // do one thing: copy the reverse of A into B.

  //reverse(mbv,mbv); // ERROR: creates two mut borrows ra,rb inside reverse
  let mut vct5 = vec![0;5]; //create vector of 5 zeros;
  reverse(mbv, &mut vct5);  // mbv is still borrowing vct4
  for x in &vct5 {print!("{} ",x);}  println!(""); // vct4 in reverse

  // Note that the &mut vct5 created by the function call was a temporary,
  // and is no longer used in the rest of main, so it's ok to create another
  // borrow of vct5 afterwards.

  // Also note that these borrowing rules applies to both :Copy and 
  // non- :Copy types:
  // let mut x = 1; let px = &mut x;  let px2 = &x;  *px += 1; //won't compile
  // let b = {let x =1; &x}; // "borrowed value does not live long enough"
                             // the {}'s limit the lifetime of x.
			     
  ///////// QUIZ!
  // Let's see if you have learned these concepts well enough.  Look at the
  // g function that's commented out at the end.  Will it compile if
  // uncommented?  How about the function h?

  // Want to know the answer to the quiz? COMPILE IT AND SEE FOR YOURSELF!


  //////////////////////// NOW FOR THE BAD PART //////////////////////////

  // I've tried so far to explain Rust using examples that are as simple as
  // I can make them.  But the truth is that Rust's ownership and borrowing
  // rules will have a major impact on how you write programs.  You have
  // to think much harder than you normally would with a language that
  // relies on a garbage collector.  Consider the following (mbv is still
  // a live mutable borrow of vector vct4):

  mbv[0] = mbv[mbv.len()-1]; //no problem, sets first value to the last value

  // we are using a single mutable borrow to access then change the
  // vector that mbv is borrowing.  No conflicts.  But now consider:

  //mbv[mbv.len()-1] = mbv[0];  // Won't compile, but why?

  // What's wrong with the above?  It's simply trying to change the
  // last value of the vector; it's something you can write without
  // any problems in other languages.  You have to really think
  // hard to understand the compiler's error message: "immutable borrow
  // occurs here ... mutable borrow later used here".  It may look like
  // we're not doing anything different from the previous line, which
  // compiled.  But there's a difference between using the expression
  // mbv[mbv.len()-1] as an 'rvalue' (on the right-hand side of =) than
  // as an 'lvalue' (on the left side of =). rvalues are temporary: they
  // are created then destroyed, while lvalues are 'names': they express
  // ownership and persist after the assignment.  The two uses of
  // mbv in mbv[mbv.len()-1] on the right-hand side of = are both immutable
  // uses: they only access and do not modify the vector. Once they are used
  // they can be forgotten: they do not interfere with the mutable use of
  // mbv[0] on the left side of =.  But when mbv[mbv.len()-1] is an lvalue
  // on the left side of =, which must persist after the assignment,
  // there is a mutable use of mbv when we change the vector, which cannot
  // coexist with the other use of mbv in 'mbv.len()'.

  // It's not hard to get around these restrictions once you understand them:

  let lastindex = mbv.len()-1;
  mbv[lastindex] = mbv[0];

  // Perhaps in the future the Rust compiler can be refined to recognize
  // some safe exceptions to its rules, to make life slightly easier for
  // programmers, but understanding the above example carefully is useful.
  
  // Rust's restrictions on ownership and borrowing are sometimes too rigid
  // that compromises must be made.  This is especially true if you want to
  // create a recursive and mutable data structure.  There are limits to
  // what static analysis can achieve (ultimately because of the 
  // undecidability of the Halting Problem).  Thus there are times when
  // you will have to use the RefCell smart pointer (which I tried to
  // simulate in C++) and Ref, RefMut in place of regular borrows.  Their
  // usage sacrifices compile-time memory checks and only give you errors
  // if you violate the borrowing rules at runtime.  There are also times
  // when even these might not be enough, and which is why Rust does
  // permit you to write 'unsafe' code, as long as it's done in isolation.

  // An important concept in high-level programming languages is
  // 'referential transparency', which basically means 'what you see
  // is what you get'. This becomes much harder to achieve with Rust.
  // If lastindex is the same as mbv.len()-1, then why isn't mbv[lastindex]
  // the sames as mbv[mbv.len()-1]?  That's a reasonable question, and 
  // it's one reason we can call Rust a 'low-level' language.  Here,
  // 'high' and 'low' are just technical terms and do not represent any
  // qualitative judgment.  If you're a typical applications-oriented
  // programmer, then you probably should just stick with languages with
  // a garbage collector.  But if you're interested in systems-level
  // programming and are as persistent as I am, then learning Rust will
  // benefit you immensely.  Even if you continue to use C/C++, it will
  // make you a much better programmer in those languages.

}//main


fn f(v:Vec<i32>) -> Vec<i32> // a function that takes and returns ownership
{
   //ownership of value passed-in moves when it's implicitly assigned to v.
   // do stuff with v ...
   let mut v2 = v;  // must define local mut var in order to change things
   v2.push(100);
   return v2;   // transer ownership of local var
}

// a function that takes borrows, and where rb must be mutable:
fn reverse(ra:&Vec<i32>, rb:&mut Vec<i32>) // actual ra can be mut or immutable
{
   if ra.len() != rb.len() {return;}
   for i in 0..rb.len() { rb[i] = ra[ra.len()-1-i]; }
}


/*  // Does the following compile if uncommented?
fn g(x:i32) -> &mut i32  //calculates a number and returns a mutable reference
{
   let cubex = x*x*x;
   return &mut cubex;  //  Type checks ok, doesn't it?
}
*/

/*   How about this function?  Will it compile?
fn h()
{
   let mut v = vec![1,2,3,4];
   let bv = &v; // immutable borrow
   { 
      let mv = &mut v; // a mutable borrow
      mv[0] = 7;
   }
   let bv2 = &v; // another immutable borrow
   for x in bv2 { print!("{} ",x); } println!("");
}
*/

// Educational Program by Chuck Liang, Hofstra University
