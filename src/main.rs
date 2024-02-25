// importing functions from another library
// Syntax: use <project-name>::<function-name>
// project name as defined in the toml file
use demo::greet; 

fn main() {
    println!("Hello, world!");

    //about_variables();
    
    // Invocation: <fn-name>(args)
    let my_val = my_test_function(21);
    println!("My value: {}", my_val);

    // invoking method from another library in the same project
    greet();
    // OR using scope operator: rust-demo::greet();
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
    let (mut radius, diameter): (f32, i32) = (3.14, 40);    // tuple destructuring
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

// Declaration: fn <name-snake-case>(<param-name1>: <type>, <param-name2>: <type>) -> <ret-type>
// can appear in no particular order in the file
// no named args at call-site
// no variable arguments to function
fn my_test_function(qty: i32) -> i32
{
    println!("{}", qty);

    return qty * 2; // OR simply qty * 2; tail expression
}
