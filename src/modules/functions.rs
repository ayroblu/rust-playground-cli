pub fn show_functions() {
    println!("Structures can be class like, by implementing static or instance methods");
    show_struct_impl();

    println!("\nClosures");
    show_closures();

    println!("\nGenerics Traits and Impl");
    show_impl();

    println!("\nPhantom data for newtype like operations");
    show_phantom_types();
}

fn show_struct_impl() {
    let rectangle = Rectangle {
        // Static methods are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable object
    //rectangle.translate(1.0, 0.0); // uncomment for error

    // Okay! Mutable objects can call mutable methods
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    //pair.destroy(); // uncomment for error
}

struct Point {
    x: f64,
    y: f64,
}

// Static methods
impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is an instance method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    fn destroy(self) {
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);
    }
}

fn show_closures() {
    let a = Some(3);
    println!("Using mapping lambdas");
    a.map(|v| v + 1).map(|v| println!("Mapped value: {}", v));

    println!("Using inline lambdas");
    fn function(i: i32) -> i32 {
        i + 1
    }

    let i = 1;
    println!("function: {}", function(i));

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i: i32| i + 1;

    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    let one = || 1;
    println!("closure returning one: {}", one());

    println!("Borrows by value T, reference &T, and mutable reference &mut T");
    let color = String::from("green");
    let print = || println!("print: `color`: {}", color);
    print();
    // Immutable so reborrows are okay
    let _reborrow = &color;
    print();
    let color_moved = color;
    println!(
        "Borrows until the function has been called all the times: {}",
        color_moved
    );
    let consume = || {
        println!(
            "Use of std:mem::drop to destroy a variable: {}",
            color_moved
        );
        std::mem::drop(color_moved);
    };
    consume();
    // consume();

    println!(
        "Complex data structures borrow reference immutably, so use `move` keyword to borrow value"
    );
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    // println!("There're {} elements in vec", haystack.len());

    println!(
        "Closures as function params, {:?}, {}",
        apply(|| println!("(first called)")),
        apply_spec(|x| x + 1)
    );

    println!("Returning closures requires an impl Fn() and a move");
    let a = create_fn();
    println!("Calling closure");
    a();

    let foo = |x| move |y| move |z| x + y + z;
    println!(
        "basic curried inline functions is straightforward: {}",
        foo(1)(2)(3)
    );

    // Functional approach
    let is_odd = |n| n % 2 == 1;
    let upper = 1000;
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // All natural numbers squared
        .take_while(|&n_squared| n_squared < upper) // Below upper limit
        .filter(|&n_squared| is_odd(n_squared)) // That are odd
        .fold(0, |acc, n_squared| acc + n_squared); // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

// Fn &T < FnMut &mut T < FnOnce T in terms of flexibility, FnOnce means it could be &T or &mut
// T, and it's up to the compiler to decide
fn apply<F: FnOnce()>(f: F) {
    f();
}

fn apply_spec<F>(f: F) -> i32
where
    // Example with argument and return type
    F: Fn(i32) -> i32,
{
    f(3)
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    println!("Creating closure to return");
    move || println!("This is a: {}", text)
}

fn show_impl() {
    println!("How Generics, traits, impl work together");

    let null = Null;
    let string = "hi";
    println!("Use impl generics to make a method available on multiple data structures");
    string.double_drop(null);
    // string.double_drop(null); // deallocated at this point

    println!("Multiple bounds with `+` symbol");
    compare_prints(&string);

    println!("using a where clause is more expressive for example when adding a wrapping type");
    let vec = vec![1, 2, 3];

    vec.print_in_option();
    println!(
        "Example of implementation of Debug using newtype idiom: {:?}",
        Years(24)
    );
}
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

// Implemented over all types
impl<T, U> DoubleDrop<T> for U {
    // This method takes ownership of both passed arguments, deallocating both.
    fn double_drop(self, _: T) {}
}

use std::fmt::{Debug, Display, Formatter, Result};
fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

struct Years(i64);
impl Debug for Years {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Years({})", self.0)
    }
}
trait Contains {
    // Associated types mean you don't need to use Contains<A, B>
    type A;
    type B;

    // Use &Self:: rather than &
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool; // Explicitly requires `A` and `B`.
    fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`.
    fn last(&self) -> i32; // Doesn't explicitly require `A` or `B`.
}
#[allow(dead_code)]
fn difference<C>(container: &C) -> i32
where
    C: Contains,
{
    container.last() - container.first()
}

use std::marker::PhantomData;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

/// `f64` already implements the `Clone` and `Copy` traits.
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn show_phantom_types() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // Compile-time Error: type mismatch.
    //let one_feter = one_foot + one_meter;
}
