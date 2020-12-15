pub fn show_control_flow() {
    println!("Control Flow: if else");
    if_else();

    println!("\n(infinite) loop");
    show_loops();

    println!("\nmatch");
    show_match();

    println!("\nif let");
    show_if_let();
}

fn if_else() {
    let n = 5;
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}
fn show_loops() {
    // Infinite loop
    let mut count = 0u32;
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("count: {}", count);
        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }

    count = 0;
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            if count == 0 {
                count += 1;
                break 'inner;
            }
            break 'outer;
        }
        println!("This point will be reached once");
    }
    println!("Exited the outer loop");

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };
    println!("Assigned loop result: {}", result);

    println!("\nwhile loop");
    count = 1;
    while count < 20 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }

        // Increment counter
        count += 1;
    }

    println!("\nfor loop");
    for n in 1..20 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
    println!("range: 1..5: {:?}", 1..5);
    println!("range: 1..=5: {:?}", 1..=5);
    println!("\niterators:");
    let names = vec!["Bob", "Frank", "Ferris"];
    println!("NOTE: the use of iter here means that it is borrowed, into_iter will consume it, and into_mut is mutable");

    println!("iter");
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("into_iter");
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // Note "moved" from into_iter, can't be used again
    //println!("{:?}", names);

    let mut names = vec!["Bob", "Frank", "Ferris"];
    println!("iter_mut");
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}
fn show_match() {
    let boolean = true;
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);
    // Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        // `..` can be the used ignore the rest of the tuple
        _ => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }

    println!("References");
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // Same as before but with ref keyword
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // rename the variables, the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }

    println!("Match guard (if after destructuring)");
    let pair = (2, -2);

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // The ^ `if condition` part is a guard
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

    println!("Range binding, use @ symbol for value in ranges");
    let age = 5;
    match age {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }

    println!("Range binding inside a destructuring");
    let a = Some(42);
    match a {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
}
fn show_if_let() {
    let letter: Option<i32> = None;
    // All the same semantics of an if statement
    if let Some(i @ 42) = letter {
        println!("Matched 42: {:?}!", i);
    } else if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let mut optional = Some(0);

    // For while too
    while let Some(i) = optional {
        if i > 3 {
            println!("Greater than 3, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}
