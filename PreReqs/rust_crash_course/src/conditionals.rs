// Conditionals are used to check the condition of something and act on the result.
// just like an if/else statement in other languages.

pub fn run() {
    let number_of_cats = 0;

    if number_of_cats > 3 {
        println!("You must be crazy!");
    } else if number_of_cats <= 2 && number_of_cats > 0 {
        println!("You have an acceptable amount of cats.");
    } else if number_of_cats == 0 {
        println!("You should get a cat.");
    }

    // Shorthand if
    let crazy_cat_lady = if number_of_cats > 3 { true } else { false };
    println!("Are you a crazy cat lady? {}!", crazy_cat_lady);
}
