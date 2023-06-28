const PI: f32 = 3.14;

fn main() {
    if_else_statement();
    match_expression();
    loop_iter();
    while_loop();
    for_loop();

    tuples();
    arrays_function();
    structures();
    structures();
    slices();
    mutable_slice();
    vectors();
    hash_maps();
    rust_module();
}

fn if_else_statement() {
    let number = 10;
    if number < 0 {
        println!("Number is negative");
    } else if number > 0 {
        println!("Number is positive");
    } else {
        println!("Number is zer");
    }
}

fn match_expression() {
    let fruit = "apple";

    match fruit {
        "apple" => println!("It's an apple"),
        "banana" => println!("It's banana"),
        _ => println!("It's something else"),
    }
}
fn loop_iter() {
    let mut count = 0;

    loop {
        println!("Current count : {}", count);
        count += 1;
        if count == 5 {
            break;
        }
    }
}

fn while_loop() {
    let mut count = 0;
    while count < 5 {
        println!("Current count: {}", count);
        count += 1;
    }
}

fn for_loop() {
    let fruits = ["apple", "banana ", "organe"];
    for fruit in fruits.iter() {
        println!("fruit: {}", fruit);
    }
}

fn tuples() {
    //tuple creation
    let person = ("Jhon", 25, true);
    //tuple access
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is Active: {}", person.2);

    //tuple destructuring
    let (name, age, is_active) = person;
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is Active: {}", is_active);

    //returnin tuples from function
    let person1 = get_person();
    println!("Name: {}", person1.0);
    println!("Age: {}", person1.1);
    println!("Is Active: {}", person1.2);
}

fn get_person() -> (String, u32, bool) {
    let name = String::from("Jhon");
    let age = 25;
    let is_active = true;
    (name, age, is_active)
}

fn arrays_function() {
    let numbers: [i32; 4] = [10, 20, 30, 40];
    println!("{:?}", numbers);

    println!("First element: {}", numbers[0]);
    println!("Second element: {}", numbers[1]);

    println!("Array Length: {}", numbers.len());

    for num in numbers.iter() {
        println!("Number: {}", num);
    }
}
struct Person {
    name: String,
    age: u32,
    is_active: bool,
}

fn structures() {
    let mut person = Person {
        name: String::from("Jhon"),
        age: 25,
        is_active: true,
    };
    // println!("{:?}", person);
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);

    person.age = 30;
    println!("Modified Age: {}", person.age);
}

fn slices() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("{:?}", slice);
}

fn mutable_slice() {
    let mut numbers = [1, 2, 3, 4, 5];
    let slice = &mut numbers[1..4];
    println!("{:?}", slice);

    slice[0] = 10;
    slice[1] = 20;
    slice[1] = 30;

    println!("{:?}", slice);
}

enum Color {
    Red,
    Green,
    Blue,
}
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

// fn enums() {
//     let red = Color::Red;
//     let green = Color::Green;
//     let blue = Color::Blue;

//     println!("{}", red);
//     println!("{}", green);
//     println!("{}", blue);

//     // print_shape_area();
// }

// enum Shape {
//     Circle(f64),
//     Rectangle(f64, f64),
//     Triangle(f64, f64, f64),
// }
// fn print_shape_area(shape: Shape) {
//     match shape {
//         Shape::Circle(radius) => {
//             let area stf::f64::consts::PI * radius*radius;
//         }
//         Shape::Rectangle(length, width) => {
//             let area = length * width;
//             println!("Area of the rectangle: {}", area);
//         }
//         Shape::Triangle(side1, side2, side3) => {
//             let s = (side1+side2+side3) / 2.0;
//             let area = (s* (s-side1)* (s-side2)* (s-side3)).sqrt();
//             println!("Area of the triangle: {}", area);
//         }
//     }
// }

fn vectors() {
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    println!("{:?}", numbers);

    let first_element = numbers[0];
    println!("First element: {}", first_element);

    for number in &numbers {
        println!("{}", number);
    }
}

use std::collections::HashMap;

fn hash_maps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 90);

    println!("{:?}", scores);

    let alice_score = scores.get("Alice");

    print!("Alice's score {:?}", alice_score);

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}

mod my_module {
    pub fn greet() {
        println!("Hello from my_module");
    }
}

fn rust_module() {
    my_module::greet();
}
