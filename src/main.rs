fn main() {
    println!("Hello, world!");

    about_variables();
}

fn about_variables()
{
    // variables MUST be initialized before use!
    // variables are immutable by default (improves concurrency, thread-safety, eliminates locks)

    // implicit type declaration
    let variable1 = 2;                      // type i32
    let (trees, animals) = (10, 150);       // tuple destructuring
    println!("{},{}, {}", variable1, trees, animals);

    // explicit type declaration
    let bunnies: i32 = 2;
    let (radius, diameter): (f32, i32) = (3.14, 40);    // tuple destructuring
    println!("{},{}, {}", bunnies, radius, diameter);

    // const declaration
    // format: const SCREAMING_SNAKE_CASE_NAME: <type> = compile-time constant value
    // inlined during compilation
    const DELTA_FACTOR: i32 = 10;
    println!("{}", DELTA_FACTOR);

    // mutable variable declaration
    let mut variable2 = 3;
    println!("{}", variable2);
    variable2 = 10;     // OK
    println!("{}", variable2);

    // mutable to immutable
    let mut y = 42;
    let y = y;
    println!("{}", y);
          
    // Scoping and shadowing rules
    let x = 4;  // valid till end of function
    {
        let x = 5; // shadows outer scope, till closing curly brace
        println!("inner scope: {}", x);
    } 
    println!("outer scope: {}", x); // value of x will be 4 again

    // another shadowing example
    let meme = 12;          // initialized as integer
    let meme = "12";        // OK, set as string
}
