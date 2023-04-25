use std::usize;


//declaration of the structs
struct Student{
    active: bool,
    email: String, //These are called fields.
    username: String,
    panther_id: usize,
    major: String,
    courses: [String;3]
}
 fn main() { 
    //Initialization
    let me = Student {
        active: true,
        email: String::from("mj.royc@blah.com"),
        username: String::from("mroyc"),
        panther_id: 1000000,
        major: String::from("CIS"),
        courses: [String::from("TCN"), String::from("COP"), String::from("CIS")]
    };
    
    //me.email = String::from("updatedemail@rust.com");

    //Create a new instance of the struct using another instance.

    let me2 = Student{
        active: me.active,
        username: me.username,
        email: String::from("mj.roy@rust.com"),
        panther_id: me.panther_id,
        major: me.major,
        courses: me.courses,
    };

    ////Create a new instance of the struct using struct update syntax
    let me3 = Student{
        email: String::from("mj.roy@rust.com"),
        ..me2
    };

    //After creating these, both first m1 and then m2 will lose its members because data have been moved.



    //tuple struct

    struct Color(usize, usize, usize);
    struct Point(usize, usize, usize);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    //Color and Point are different custom types, you cannot pass a Color instead of a Point although they are built with same data types.

 }

 fn build_student(email: String, username: String, id: usize, major:String, course1: String, course2: String, course3:String) -> Student{
    Student { active: true,  
            email,
            username,
            panther_id: id, 
            major,
            courses: [String::from(course1), String::from(course2), String::from(course3)] 
        }
 }
