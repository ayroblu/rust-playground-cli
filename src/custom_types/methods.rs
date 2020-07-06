pub fn show_custom_types() {
    show_structs();
    show_enums();
    show_constants();
}
fn show_structs() {
    println!("Struct 1: Tuple structs");
    println!("Pair: {:?}", Pair(32, 32.));

    println!("\nStruct 2: Classic structs");

    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    let james = Person {
        name: "James",
        age: 28,
    };
    println!("peter: {:?}", peter);
    println!("james: {:?}", james);

    let point_a = Point { x: 1., y: 2. };
    let point_b = Point { x: 1.5, ..point_a };
    println!("point_b: {:?}", point_b);
    let Point { x: point_b_x, y: _ } = point_b;
    println!("point_b x: {}", point_b_x);

    println!("\nStruct 3: Unit structs");
    println!("{:?}", Unit)
}

#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
#[derive(Debug)]
struct Unit;

// A tuple struct
#[derive(Debug)]
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn show_enums() {
    println!("Hi");
}
fn show_constants() {
    println!("Hi");
}
