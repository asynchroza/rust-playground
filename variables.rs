use std::mem::drop;
use std::convert::TryInto;

#[derive(Copy, Clone)]
struct Foo(u8);

fn main() {

    // mut (to make it mutable)
    let spaces = "    ";
    println!("spaces var has len({})",spaces.len());

    // integer types in rust
    let x: isize = 123903129;
    println!("x var has a value of {}", x);

    //mem::drop(x); // transfering ownership out of scope with std::mem
    let mut x:i8 = 5;
    println!("x var has a value of {}", x);

    let y = Foo(x.try_into().unwrap());
    let ref_x = &x;
    println!("referenced x is {}", ref_x);

    // https://doc.rust-lang.org/std/mem/fn.drop.html
    // none of those are not implicitly copyable
    drop(x);
    drop(ref_x);
    drop(y);
    
    // https://www.reddit.com/r/rust/comments/2inyyo/comment/cl412xf/?utm_source=share&utm_medium=web2x&context=3
    //
    // NOT IMPLICITIBLY COPYABLE?
    
    // Mutable references (&mut).
    // Anything that has a destructor (i.e. implements the Drop trait).
    // Anything that can be shared across tasks (i.e. implements the Send trait (is it still called that? I've been away for a while)).
    // Anything that is a container for any of the above (like a struct with a &mut member).
    
    println!("x: {  }, ref_x: {  }, y: {  }", x, ref_x, y.0);
    
    let mut test_vec = Vec::new();
    for i in 1..1000000 {
        test_vec.push(i);
    }
    drop(test_vec);
    // println!("{}", &test_vec.len()); ---> value borrowed because it is moved to another stack
    // when dropped
}
