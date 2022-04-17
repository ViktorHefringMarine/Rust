

fn print_type_of<T>(_: &T) {
    println!("print_type_of ->   {}", std::any::type_name::<T>())
}


fn run(what:i8, something:&str) {
    println!("----");
    println!("Guess the number!");
    println!("Please input your guess:");

    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.push_str("Whaat");


    println!("You guessed: {}, {}, {}", guess, what, something);

    print_type_of(&guess);
}

fn main() {
    run(3,"jk");
    run(3,"aj");
}

