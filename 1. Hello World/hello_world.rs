// This is a comment, like many other languages

// Declare a function using fn
fn main() {
    println!("Hello World!");
    println!("I'm a Rustacean!");

    // Lots of printing utils are defined in std::fmt.
    // Here's some of them

    // In general, using {} in a string will allow for arguments
    // to be substituted into strings. The arguments will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside
    // `{}` determines the index of the variable argument list provided to be used.
    // Arguments start at index 0 right after the format string.
    println!("Hi! My name is {0}, and I'm a {1} at {2}", "Walter White", "Chemistry Teacher", "J.P. Wynn High School");

    // You can also use named arguments
    println!("My name is {name}, yo. My husband is {husbands_name}, yo. Uh huh!", name="Skylar White", husbands_name="Walter White");

    // Different formatting can be used by specifying the format character after a :
    println!("Base 10:               {val}", val=69420);
    println!("Base 2 (binary):       {val:b}", val=69420);
    println!("Base 8 (octal):        {val:o}", val=69420);
    println!("Base 16 (hexadecimal): {val:x}", val=69420);

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number=1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi: f64 = 3.14159265;
    println!("Pi to 5 decimal places: {piVar:.5}", piVar=pi);
}