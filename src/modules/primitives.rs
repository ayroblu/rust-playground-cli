pub fn show_primitives() {
    println!("i128: min: {}, max: {}", i128::MIN, i128::MAX);
    println!("i64: min: {}, max: {}", i64::MIN, i64::MAX);
    println!("i32: min: {}, max: {}", i32::MIN, i32::MAX);
    println!("i16: min: {}, max: {}", i16::MIN, i16::MAX);
    println!("i8: min: {}, max: {}", i8::MIN, i8::MAX);
    println!("isize: min: {}, max: {}", isize::MIN, isize::MAX);

    println!("\nUnsigned");
    println!("u128: min: {}, max: {}", u128::MIN, u128::MAX);
    println!("u64: min: {}, max: {}", u64::MIN, u64::MAX);
    println!("u32: min: {}, max: {}", u32::MIN, u32::MAX);
    println!("u16: min: {}, max: {}", u16::MIN, u16::MAX);
    println!("u8: min: {}, max: {}", u8::MIN, u8::MAX);
    println!("usize: min: {}, max: {}", usize::MIN, usize::MAX);

    println!("\nFloat");
    println!("f64: min: {:e}, max: {:e}", f64::MIN, f64::MAX);
    println!("f32: min: {:e}, max: {:e}", f32::MIN, f32::MAX);

    println!("\nChar");
    println!("char: {}", '\u{00e9}');
    // One byte, not one bit
    println!("\nBool");
    println!("bool: size: {}", std::mem::size_of::<bool>());

    println!("\nUnit");
    println!("unit type: {:?}", ());

    println!("\nCompound Types");
    println!("{:?}", [1, 2, 3]);
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];
    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    // `len` returns the size of the array
    println!("array size: {}", xs.len());
    // length 3
    println!("ys: {:?}", &ys[1..4]);

    println!("{:?}", (1, true));
    // Can be up to length 12
    let t = (1, 'a', "yes");
    // Must destructure fully
    let (one, a, yes) = t;
    println!("{:?}: tuple member 2: {}", t, t.2);
    let rev_t = (yes, a, one);
    println!("rev_t: {:?}", rev_t);
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    println!("\nTypes");
    // Defaults i32, f64
    println!("size of default int (i32): {}", std::mem::size_of_val(&1));
    println!(
        "size of default float (i64): {}",
        std::mem::size_of_val(&1.)
    );

    println!("\nAnnotations");
    let f: f32 = 1.;
    let i = 1i64;
    println!("size of annotated float: {}", std::mem::size_of_val(&f));
    println!("size of annotated int: {}", std::mem::size_of_val(&i));

    // Variables can be overwritten with shadowing.
    let i = true;
    println!("size of shadowed i: {}", std::mem::size_of_val(&i));

    println!("\nNotations");
    println!("0x1f: {}", 0x1f);
    println!("0o10: {}", 0o10);
    println!("0b10: {}", 0b10);
    println!("1_000.000_1: {}", 1_000.000_1);

    println!("\nBitwise operations");
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("\nCasting");
    let decimal = 65.8321_f32;
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    // Not allowed
    // println!("1000 as a u8 is : {}", 1000 as u8);
    println!("  -1 as a u8 is : {}", (-1i8) as u8);
}
