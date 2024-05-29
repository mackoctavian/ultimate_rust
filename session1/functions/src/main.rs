fn double(n: i32) -> i32 {
    n * 2
}

fn double_or_nothing(n: i32) -> i32 {
    if n > 0 {
        return n * 2;
    }

    0
}

fn greet(s: String) {
    println!("Hello {s}");
}

//Function to borrow a value
fn greet_borrow(s: &String) {
    println!("{s}");
}

//Function to change the mutable value
fn greet_borrow_mut(s: &mut String) {
    *s = format!("Hello {s}");
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Not working");
    input.trim().to_string()
}

fn main() {
    println!("{}", double(5));

    let i = 5;
    let n = if i == 5 { 6 } else { 7 };

    let name = "Mack".to_string();
    let mut name1 = "Mack".to_string();
    greet(name.clone()); //Copying the value of name to greet fn
    greet_borrow(&name); //Borrowing the value of name
    greet_borrow_mut(&mut name1); //Borrowin and mutating the value of name1

    let input: String = read_line();
    println!("You typed: {input}")
}
