fn main() {
    println!("Hello, world!");

    // ------------------------
    println!("\n");
    let mut a = 100;
    a = 110;

    let b: i32 = 200;
    let c: i32 = 300;

    println!("The number is: {}, {}, {}", a, b, c);

    // ------------------------
    println!("\n");
    let f: f64 = 3.14;
    println!("The float number is: {}", f);

    // ------------------------
    println!("\n");
    let is_active: bool = true;
    let is_finished: bool = false;

    // println!("Is active? {}", is_active);
    // println!("Is finished? {}", is_finished);

    let comparison: bool = 10 > 5;
    println!("Is 10 greater than 5? {}", comparison);

    // ------------------------
    println!("\n");
    let ch: char = 'R';
    println!("Character is: {}", ch);

    // ------------------------
    println!("\n");
    const x: i32 = 100;
    println!("The number is: {}", x);

    // ------------------------
    println!("\n");
    let mut name: String = String::from("Ruchit");
    let name2: String = "Ruchit".to_string();
    println!("Name: {}, Name2: {}", name, name2);

    // ------------------------
    println!("\n");
    let mut vec: Vec<u64> = Vec::new();
    vec.push(10);
    vec.push(15);

    let length_vec: usize = vec.len();

    println!("Vector/Array is: {:?} and it's length is: {}", vec, length_vec);
    
    // ------------------------
    println!("\n");
    struct Rectangle {
        length: u32,
        width: u32,
    }

    let my_rectangle: Rectangle = Rectangle {
        length: 10,
        width: 20,
    };

    println!("Length and Width of my rectangle is: {} and {}", my_rectangle.length, my_rectangle.width);

    // ------------------------
    println!("\n");
    let my_string: String = "Ruchit".to_string();
    let other_string: &String = &my_string;

    println!("My String: {}", my_string); 
    println!("Other String: {}", other_string);

    // ------------------------
    println!("\n");
    let sum: u32 = sum(10, 20);
    println!("Sum of two numbers is: {}", sum);

    let sub: u32 = sub(20, 10);
    println!("Substraction of two numbers is: {}", sub);

    let mul: u32 = mul(2, 2);
    println!("Multiplication of two numbers is: {}", mul);
    
    let div: u32 = div(10, 3);
    println!("Division of two numbers is: {:.2}", div);

    let modulo: u32 = modulo(10, 3);
    println!("Modulo of two numbers is: {}", modulo);
}

fn sum(a:u32, b:u32) -> u32 {
    let sum: u32 = a + b;
    return sum;
}

fn sub(a:u32, b:u32) -> u32 {
    a - b
}

fn mul(a:u32, b:u32) -> u32 {
    a * b
}

fn div(a:u32, b:u32) -> u32 {
    a / b
}

fn modulo(a:u32, b:u32) -> u32 {
    a % b
}