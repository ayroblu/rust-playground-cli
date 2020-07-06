use std::f64::consts::PI;

/// Note rust is not locale dependent at all
pub fn show_print() {
    println!("Hi");
    println!("1/3 == {}", 1 / 3);
    println!("1./3. == {}", 1. / 3.);
    let fmt_str = format!("text: {}", 23);
    println!("{}", fmt_str);
    print!("some stuff here: ");
    println!("result + newline");
    eprintln!("This goes to standard error");
    eprint!("same line stderr\n");

    println!("\n# Precision");
    println!("pi is roughly: {:.3}", PI);
    // .* means first is usize, second is value
    println!("Hello {} is {:.*}", "x", 5, 0.01);
    println!("Hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);
    println!(
        "{}, `{name:.*}` has 3 characters",
        "Hello",
        3,
        name = "1234.56"
    );

    // Actual formatting
    println!("\n# Formatting substitution");
    println!("{0} + {1} = {0}{1}", "a", "b");
    println!("{1} {} {0} {}", 1, 2); // 2 1 1 2
    println!(
        "{name}: {description}",
        name = "MyName",
        description = "describes MyName"
    );

    // Special formatting can be specified after a `:`.
    println!("\n# Special formatting options");
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );
    println!("Debug this option: `{:?}`", Some(3));
    // Exponent
    println!("e option `{:e}`", 30.333);
    println!("E option `{:E}`", 30.333);
    // octal
    println!("o option `{:o}`", 30);
    let thing = 3;
    // pointer
    println!("p option `{:p}`", &thing);
    // base 2 (binary)
    println!("b option `{:b}`", 30);
    // hex
    println!("x option `{:x}`", 30);
    println!("X option `{:X}`", 30);

    println!("\n# Width formatting");
    // Pad out binary with 0s
    println!(
        "{:08} of {:08b} people know binary, the other half doesn't",
        1, 2
    );
    // All of these print "Hello x    !"
    println!("Hello {:5}!", "x");
    // Note that the $ means usize width
    println!("Hello {:1$}!", "x", 5);
    println!("Hello {1:0$}!", 5, "x");
    println!("Hello {:width$}!", "x", width = 5);

    println!("\n## Alignment formatting");
    println!("Hello {:<5}!", "x"); // left
    println!("Hello {:-<5}!", "x");
    println!("Hello {:^5}!", "x"); // center
    println!("Hello {:>5}!", "x"); // right
    println!("Hello {:^15}!", format!("{:?}", Some("hi"))); // => "Hello   Some("hi")   !"
    println!("{number:>width$}", number = 1, width = 6);
    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number = 1, width = 6);
    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number = 1, width = 6);

    println!("\n## Sign");
    println!("Hello {:+}", 5);
    // pretty debug
    println!("{:#?}", Some(Some(27)));
    // Precede with 0x
    println!("{:#x}", 27);
    println!("{:#X}", 27);
    // Binary, octal
    println!("{:#b}", 27);
    println!("{:#o}", 27);
    // Note negatives
    println!("Hello {:02}", 5);
    println!("Hello {:02}", -5);
    // # (alt) 0 (prepend) 10 (10 times) x (hex 0x)
    println!("{:#010x}", 27);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    // println!("My name is {0}, {1} {0}", "Bond");

    // println!("This struct `{}` won't print...", Some(3));
    println!("\n# Escaping");
    println!("{{ Hello }}");
}
