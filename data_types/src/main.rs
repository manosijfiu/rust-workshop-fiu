fn main() {

    /*
    SCALAR Types
    */
    //uint, int  = Unsigned, Signed Integers : u8/i8, u16/i16, u32, u64 -2^7 to (+2^7 - 1) : -128 to +127 11111111 + 1 = 255 u8 = 256
    let guess: u32 = "42".parse().expect("Not a number!");

    let maxint:u8 = 255;

    println!("The maximum value of the int is = {maxint}");



    //let guess = "42".parse().expect("Not a number!"); //Will give a compilation error.. to which data type you want to cast?

    //usize and u32 is same - default for rust

    //Rust does not check integer overflow and panics, it wraps around. for u8 the 256 becomes 0 and 257 becomes 1.

    //Only when compiling with --release flag.


    //f32, f64 = Floating point numbers.

    let x = 2.23;// f64 - Default - Double Precision, required for scientific calculators and minimization of approximation
    let floatValue = 1.0;

    let y: f32 = 3.0; // f32 Single Precision, required for normal activities and gamings.

    //bool = boolean types

    let t = true;

    let f: bool = false; // with explicit type annotation 

    //char character types - enclosed by single quotes. 4 bytes Unicode Scalar value.

    let c = 'z'; 
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻'; //


    /*
    Compund Types
     */

    //tuples - A single compund element. it can store several values with different data types.

    let tup: (usize, char, bool) = (12, 'D', true);

    let tup = (12, '&', 23.12); //shadowing

    let tuptest = (100, 'M', 23.4);

    let (a, b, c) = tuptest; //destructuring

    println!("The first value of tuptest is: {a}");
    println!("The second value of tuptest is: {b}");
    println!("The third value of tuptest is: {c}");

    let (x, y, z) = tup; //destructuring (Similar to ES6)

    println!("The value of y is: {y}");

    let first = tup.0;
    let second = tup.1;
    let third = tup.2;

    println!("The first value of tup is: {first}");
    println!("The second value of tup is: {second}");
    println!("The third value of tup is: {third}");

    //Array - Every element of an array must have one data type. If we do not explicitly mention the data type, the array 
            // will assume the elements to be the data type of first element and will throw compilation error in case of 
            //a presence of any other values.

            //Usefull when you want your data allocated on the stack rather than the heap. Not flexible as Vector. 
            // Useful when the size is pre-determined.

    let arr = [2, 3, 4, 9, 6];

    let arrtest: [u8; 10] = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90];

    let first = arrtest[0];
    let second = arrtest[4];
    let ninth = arrtest[8];

    println!("The first value of array is: {first}");
    println!("The second value of array is: {second}");
    println!("The third value of array is: {ninth}"); //index out of bound exception - runtime.

    let arr = [3;9];
    
    println!("The first value of tup is: {:?}", arr); //to print the array. 




}
