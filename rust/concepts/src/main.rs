fn type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn main() {

    // mutable variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("Now, the value of x is {x}");

    // Constants
    // constants are always immutable.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours is {THREE_HOURS_IN_SECONDS} seconds.");

    // Shadowing
    // We can shadow a variable by using the same variable`s name and repeating the use of the let keyword of follows:
    let y = 5;
    
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y out of the inner scope is: {y}");

    // Data types
    // Rust is a statically typed languague, which means that it must know the types of all variables at compile time

    let guess: u32 = "42".parse().expect("Not a number!");
    type_of(&guess);

    // let guess = "42".parse().expect("Not a number!"); <- this line produce an error and inform " consider giving `guess` an explicit type"
    
    // >Scalar Types
    // A scalar type represents a single value. Rust has four primary scalar types: intergers, floating-point numbers, booleans and characters.

    // >>Interger Types
    // The interger types can be signed or unisigned, from 8 to 128 bits, or arch.
    // The signed or unsigned refers to math signals, for example, a negative number, shoulb be of signed type.
    // The arch time is related to the architeture of the system that will run the code, can be 64 or 32.

    let negative_128_number: i128 = "-42".parse().expect("Not a number!");
    type_of(&negative_128_number); //if we pass u128 we get an error.

    // >>Floating-Point Types
    // Rust has two types for floaing, with 32 or 64 bits in size: f32 and f64
    // The f32 type is a single-precision float, and f64 has double precision.
    let x_float = 2.0;
    let y_float: f32 = 3.0;
    type_of(&x_float);
    type_of(&y_float);

    // >>Numeric Operations
    // Rust supports the basic mathematical operations.
    let sum = 5 + 10;
    let sub = 95.5 - 4.3;
    let mult = 4 * 30;
    let div = 56.7 / 32.3;
    let div_2 = 2 / 3;
    let rem = 43 % 5;
    println!("5 plus 10 is equal to {sum}");
    println!("95.5 minus 4.3 is qual to {sub}");
    println!("4 multiplied by 30 is qual to {mult}");
    println!("56.7 divided vy 32.3 is equal to {div:.3}");
    println!("2 divided by 3 is qual to {div_2}");
    println!("The reainder of 43 divided by 5 is qual to {rem}");

    // >>The boolean type
    // Same idea of other lenguages
    let t = true;
    let f = false;
    type_of(&t);
    type_of(&f);

    // >>The character type
    // Same idea of other languages
    let c = "z";
    type_of(&c);

    // >Compound Types
    // Compound types can group multiple values into one type. It`s the same idea of dictionary in python.
    // Rust has two primitive compound types: tuples and arrays.
    let tup: (i32, f64, u8, &str) = (500, 6.4, 1, "oi");
    type_of(&tup);
    let (a, b, c, d) = tup;
    println!("The value of d is: {d}");

        
}