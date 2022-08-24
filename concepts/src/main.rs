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

    println!("The value of y is: {y}");

    // Data types
    // Rust is a statically typed languague, which means that it must know the types of all variables at compile time

    let guess: u32 = "42".parse().expect("Not a number!");
    type_of(&guess);

    // let guess = "42".parse().expect("Not a number!"); <- this line produce an error and inform " consider giving `guess` an explicit type"
}