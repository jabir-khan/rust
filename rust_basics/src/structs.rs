// Structs - Used to create custom data types

// Traditional struct
// struct Color{
//     red: u8,
//     green:u8,
//     blue:u8
// }

// Tuple struct
// struct Color(u8,u8,u8);


struct Person{
    first_name: String,
    last_name: String
}


impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name ( its method )
    fn full_name(&self) -> String { //this function returns a string
        format!("{} {} ", self.first_name, self.last_name)
    }

    //set last name
    fn set_last_name(&mut self, last: &str){
        self.last_name =  last.to_string();
    } 

    // Name to tuple
    fn to_tuple(self) -> (String, String) { // this function returns tuple
        (self.first_name, self.last_name)
    } 

}




pub fn run(){
    // let mut c  = Color{
    //     red: 255,
    //     green:0,
    //     blue:0
    // };
    // // directly access the property of struct
    // c.red = 200;

    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    // let mut c = Color(255, 0, 0);
    // c.0 = 200;
    // println!("Color: {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("John", "Doe");
    println!("Person is: {} {}", p.first_name, p.last_name);
    println!("Person full name is: {}", p.full_name());

    p.set_last_name("Williams");
    println!("Person full name after changing last name is: {}", p.full_name());

    println!("Person tuple: {:?}", p.to_tuple());


    
}