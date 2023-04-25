fn main() {
    let x = 5;

    let x = x + 1; // This is called variable shadowing

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); //The shadowing of this curly braces code sticks with its own scope
    }

    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let spaces = "   ";
    let spaces = spaces.len();

    /* 
    // This will produce compilation error
    let mut spaces = "   ";
    spaces = spaces.len();
    */
}
