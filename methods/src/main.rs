/*
Methods are similar to the functions: they are declared, has parameters, returns value(s) and invoked from other 
part of the code. Only difference is method's scope is valid within a struct

 */

struct Rectangle {
    width: usize,
    length: usize,
}

impl Rectangle {    
    fn area(&self) -> usize {
        self.width * self.length
    }

    fn width(&self) -> usize {
        self.width
    }
}

fn main() {
    let rec = Rectangle {
        width: 30,
        length: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rec.area()
    );

}
