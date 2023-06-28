fn main() {
    let company_string = "BlockchainedIndia";
    let rating_float = 4.5;
    let is_growing_boolean = true;
    let icon_char = '😃';

    println!("Company name: {}", company_string);
    println!("Company rating {}", rating_float);
    println!("Is company growing: {}", is_growing_boolean);
    println!("Company Icon: {}", icon_char);

    integer_scalar_types();
    floating_point();
    boolean_types();
    character_types();
    variables();
    constants();
    create_new_string();
    string_is_empty();
    lenght_of_string();
    string_chars();
    string_to_uppercase();
    operators();
    assignment_operators();
    comparison_operators();
    bitwise_operators();
    range_operator();
    deref_operator();
    indexing_operator();
    square_of_a_number();
    call_by_reference();
    call_by_value();
}

fn integer_scalar_types() {
    let result = 10;
    let age: u32 = 20;
    let sum: i32 = 5 - 15;
    let mark: isize = 10;
    let count: usize = 30;

    println!("Result value: {}", result);

    println!("Sum is {} and age is {}", sum, age);
    println!("Mark is {} and count is {}", mark, count);
}

fn floating_point() {
    let result = 10.00;
    let interest: f32 = 8.35;
    let cost: f64 = 15000.600;

    println!("result value is {}", result);
    println!("interest is {}", interest);
    println!("cost is {}", cost);
}

fn boolean_types() {
    let is_fun: bool = true;

    println!("Is Rust Programming Fun? {}", is_fun);
}

fn character_types() {
    let special_character = "@";
    let alphabet: char = 'A';
    let emoji: char = '😃';

    println!("special character is {}", special_character);
    println!("alphabet is {}", alphabet);
    println!("emoji is {}", emoji);
}

fn variables() {
    let count = 5;
    println!("Count: {}", count);
    let count = count * 2;
    println!("Updated count: {}", count);
}
const PI: f32 = 3.14;
const MAX_ATTEMPTS: u8 = 5;
fn constants() {
    println!("Value of PI: {}", PI);
    println!("Maximum attempts: {}", MAX_ATTEMPTS);
}

fn create_new_string() {
    let hello = String::from("hello,");
    let name = "Alice";
    let greeting = hello + name;

    println!("{}", greeting);
}

fn lenght_of_string() {
    let message = String::from("Hello, Rust");
    let length = message.len();
    println!("Length: {}", length);
}

fn string_is_empty() {
    let message = String::from("");
    let is_empty = message.is_empty();

    println!("Is empty: {}", is_empty);
}

fn string_chars() {
    let message = String::from("Hello");

    for c in message.chars() {
        println!("{}", c);
    }
}

fn string_to_uppercase() {
    let message = String::from("Hello, Rust");

    let uppercase = message.to_uppercase();
    println!("Uppercase: {}", uppercase);
}

fn operators() {
    let a = 5;
    let b = 3;

    let sum = a + b;
    let difference = a - b;
    let product = a * b;
    let quotient = a / b;
    let remainder = a % b;
    let negation = -a;

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);
    println!("Negation: {}", negation);
}

fn comparison_operators() {
    let a = 5;
    let b = 3;

    let equals = a == b;
    let not_equals = a != b;
    let greater_than = a > b;
    let greater_than_or_equal = a >= b;
    let less_than = a < b;
    let less_than_or_equal = a <= b;

    println!("Equals: {}", equals);
    println!("Not Equals: {}", not_equals);
    println!("Greater than: {}", greater_than);
    println!("Greater than ot Equal: {}", greater_than_or_equal);
    println!("Less Than: {}", less_than);
    println!("Less Than or Equal: {}", less_than_or_equal);
}

fn assignment_operators() {
    let mut a = 5;

    a += 3;
    println!("a:{}", a);
    a -= 2;
    println!("a:{}", a);
    a *= 4;
    println!("a:{}", a);
    a /= 2;
    println!("a:{}", a);
    a %= 3;
    println!("a:{}", a);
}

fn bitwise_operators() {
    let a = 0b1010;
    let b = 0b1100;
    let and_result = a & b;
    let or_result = a | b;
    let _xor_result = a ^ b;
    let not_a = !a;

    println!("AND Result: {:b}", and_result);
    println!("OR Result: {:b}", or_result);
    println!("OR Result: {:b}", or_result);
    println!("NOT A Result: {:b}", not_a);
}

fn range_operator() {
    for num in 1..5 {
        println!("{}", num);
    }

    for num in 1..=5 {
        println!("{}", num);
    }
}

fn deref_operator() {
    let number = 42;
    let number_ref = &number;
    println!("{}", *number_ref);
}

fn indexing_operator() {
    let fruits = ["apple, banana", "orange"];
    println!("{}", fruits[1]);
}

fn square_of_a_number() {
    let number = 5;
    let square_result = square(number);

    println!("Square of {} is {}", number, square_result);
}

fn square(num: i32) -> i32 {
    let _result = num * num;
    return _result;
}
fn add_one(num: i32) -> i32 {
    num + 1
}

fn call_by_value() {
    let number = 5;
    let result = add_one(number);
    println!("Original value: {}, Result: {}", number, result);
}

fn add_one_by_ref(num: &mut i32) {
    *num += 1;
}

fn call_by_reference() {
    let mut number = 5;
    add_one_by_ref(&mut number);
    println!("Modified balue: {}", number);
}
