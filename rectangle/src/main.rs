#[derive(Debug)]
struct Rectangle{
    width:u16,
    height:u16
}

fn main() {
    /* 
    //Plain Simple
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height));
    */

    
    //better approach , but not so clear
    /* 
    let rec = (30, 50);
    println!(
        "Better Approach The area of the rectangle is {} square pixels.",
        area(rec)
    );
    */
     

    
    let rec = Rectangle {
        width : dbg!(10 * 2),
        height : 30
    };
    

    //println!("rectangle is {:#?}", rec); //Compilation Error without the derive statement
    // The debug trait enables us to print a struct in customized way.
    //println!("rectangle is {:#?}", rec); //Compilation Error without the derive statement

    //println! takes reference of an expression as opposed to dbg!

    
    dbg!(&rec);

    println!(
        "Struct Approach The area of the rectangle is {} square pixels.",
        area(&rec) //The main does not lose ownership of the struct and it can do whatever it can do with the struct
    );
    

    
    
}


//Plain Simple
/* 
fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/



//better approach , but not so clear
/*
fn area(dimentions:(u32,u32)) -> u32 {
    dimentions.0 * dimentions.1
}
*/

fn area(rectange:&Rectangle) -> u16 {
    rectange.width * rectange.height
}