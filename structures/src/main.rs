fn main() {
    let mut user1 = User {  // initialization of a `User` structure as a variable
                            // structure fields can be initialized in any order
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");     // modification of a structure field
                                                                // the entire structure instance must be mutable
                                                                // syntax does not allow for mutable fields

    // let user2 = User { // some of the field values of user2 are copied from the fields of user1
    //     active: user1.active,
    //     username:user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // }; 
    
    // let user3 = User { // `..user1` moves the remaining field values from user1 into user3 
    //     email: String::from("another@example.com"),
    //     ..user1     // the `Structure Update Syntax` expression has to be at the end of the structure initialization.
    // };              // since the field key `username` type implements the `copy` trait, the structure update syntax
    //                 // moves the value of username from user1 to user3, which makes the user1 structure invalid, since
    //                 // since it no longer has a value for the key username.

    let user4 = User {
        email: String::from("another@example.com"),
        username: String::from("someusername1234"),
        ..user1         // the remaining fields `active` and `sign_in_count` are copied not moved, so no issues here
    };


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

struct User {
    active: bool,           // implements the `copy` trait
    username: String,       // implements the `move` trait
    email: String,          // implements the `move` trait
    sign_in_count: u64,     // implements the `copy` trait
}

fn build_user(email: String, username: String) -> User {    // naming the parameters of function the same as the
                                                            // keys of the fields eliminates repetition of key and
                                                            // field names, as the compiller will match the name of
                                                            // the parameter with the names of the fields, if they
                                                            // are the same
    User { // this is an expression that returns a `User` instance
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


// tuple structures: tuple structures do not require any field keys, only the field types for practicality
// the `.` operator can be used to access field values. How?
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);    // this tupe structure is not interchangeable with the tuple structure above since
                                // they are of different types even though they have the same field values

// unit-like structures: usefull for implementations that do not rely on data, such as traits                                
struct AlwaysEqual;                                