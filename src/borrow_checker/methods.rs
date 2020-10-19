pub fn show_borrow_checker() {
    println!("## Borrow Checker");

    // Rust can only read from one variable name at a time
    let x = String::from("hello");
    let y = x;
    // Change this to x for error
    println!("{}", y);

    let x = "hello";
    let y = x;
    println!("doesn't apply to static strings: {}, {}", x, y);

    let x = String::from("hello");
    let y = &x;
    println!("using a reference: {}, {}", x, y);

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let (v3, v4, answer) = foo_no_borrow(v1, v2);
    // Can't access v1, v2
    println!("no borrowing: {:?}, {:?}, {}", v3, v4, answer);

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let answer = foo_borrow(&v1, &v2);
    println!("borrowing: {:?}, {:?}, {}", v1, v2, answer);

    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    // We can only access this because the mutable borrow ends before this print
    println!("mutable borrow: was 5, now: {}", x);

    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![1, 2, 3];

    let answer = foo_borrow_mutable(&mut v1, &mut v2);
    println!("mutable borrowing: {:?}, {:?}, {}", v1, v2, answer);

    println!("\nRules:");
    println!("1. Cannot borrow in scope bigger than the owner");
    println!("2. One of: Only normal readonly references or exactly 1 mutable reference");

    println!("\nExamples:");
    println!("1. Iterator Invalidation");
    let v = vec![1, 2, 3];

    for i in &v {
        println!("- {}", i);
        // Can't do this
        // v.push(34);
    }

    println!("2. use after free");

    // let y: &i32;
    // {
    //     let x = 5;
    //     // x doesn't live long enough to assign to
    //     y = &x;
    // }
    // println!("- {}", y);
    show_lifetimes();
}

fn foo_no_borrow(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    (v1, v2, 42)
}
fn foo_borrow(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    println!("foo_borrow (readonly): {:?}, {:?}", v1, v2);
    42
}
fn foo_borrow_mutable(v1: &mut Vec<i32>, v2: &mut Vec<i32>) -> i32 {
    println!("foo_borrow_mutable: {:?}, {:?}", v1, v2);
    v1.push(456);
    42
}

// Constants
static FOO: i32 = 5;

fn show_lifetimes() {
    println!("\n## Lifetimes");
    let mut p1 = 1;
    let p2 = 2;
    bar(&mut p1, &p2, 5);

    let foo_num: &'static i32 = &FOO;
    let hello_world: &'static str = "Hello, world.";
    println!(
        "Static, baked in to the binary: {}, {}",
        foo_num, hello_world
    );
}
fn bar<'a, 'b>(p1: &'a mut i32, p2: &'b i32, p3: i32) {
    println!("bar: {}, {}, {}", p1, p2, p3)
}
