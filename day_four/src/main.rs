use my_crate::add_one;

fn main() {
    ownership_example();
    borrowing_example();
    mutable_borrowing();
    traits_example();
    use_module();
}

fn ownership_example() {
    let s = String::from("Hello");

    println!("{}", s);

    //owner ship transfer

    let s2 = s;
    println!("{}", s2);
}

fn borrowing_example() {
    let s = String::from("hello");

    let len = calculate_length(&s);

    println!("Length of '{}' is {}", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mutable_borrowing() {
    let mut s = String::from("hello");

    change_string(&mut s);
    print!("Modified string: {}", s);
}

fn change_string(s: &mut String) {
    s.push_str(", World");
}

trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

fn draw_shape(shape: &dyn Drawable) {
    shape.draw();
}
fn traits_example() {
    let circle = Circle { radius: 5.0 };
    draw_shape(&circle)
}

fn use_module() {
    println!("{:?}", add_one(20));
}
