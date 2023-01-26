// Structs in Rust are similar to classes
// Structs have attributes related to them
// Structs are used to create custom data types

// A traditional struct called Color
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Create a tuple struct called ColorTuple
struct ColorTuple(u8, u8, u8);

// define a stuct called Person with first_name and last_name attributes
struct Person {
    first_name: String,
    last_name: String,
}

// implement the person struct
impl Person {
    // create a new person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut color = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    // redefine the red value
    color.red = 210;

    // print the values of the traditional color struct
    println!("Color: {} {} {}", color.red, color.green, color.blue);

    // ColorTuple section

    // define a color_tuple variable of a ColorTuple struct data type
    let mut color_tuple = ColorTuple(210, 0, 0);

    // print the values of the ColorTuple struct
    println!(
        "Values for the ColorTuple: {} {} {}",
        color_tuple.0, color_tuple.1, color_tuple.2
    );

    // person section

    // create a new person
    let mut person = Person::new("John", "Doe");
    println!("Person: {} {}", person.first_name, person.last_name);

    person.set_last_name("Smith");
    println!("Person: {}", person.full_name());
    println!("Person Tuple: {:?}", person.to_tuple());
}
