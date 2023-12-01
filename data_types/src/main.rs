fn main() {
    // integers
    println!("------------------------------ integers ------------------------------");
    let x: i8 = 10;
    
    println!("{}", x);
    
    let y: u8 = 10;

    let decimal: i32 = 02_55;
    let hex: i32 = 0xff;
    let octal: i32 = 0o377;
    let binary: i32 = 0b1111_1111;

    println!("{}", y);
    println!("{}", decimal);
    println!("{}", hex);
    println!("{}", octal);
    println!("{}", binary);
    
    let byte = b'A';
    println!("{}", byte);
    
    // floating point
    println!("------------------------------ floating point ------------------------------");
    let float_x: f64 = 2.0; // f64 default
    let float_y: f32 = 1.0;
    println!("{}", float_x);
    println!("{}", float_y);
    
    // boolean
    println!("------------------------------ boolean ------------------------------");
    let t: bool = true;
    let f: bool = false;
    println!("{}", t);
    println!("{}", f);

    // string
    println!("------------------------------ string ------------------------------");
    let c: char = 'c';
    println!("{}", c);

    // arithmetics
    println!("------------------------------ arithmetics ------------------------------");
    let math_a: i32 = 10;
    let math_b: i32 = 5;
    
    let addition: i32 = math_a + math_b;
    let subtraction: i32 = math_a - math_b;
    let multiplication: i32 = math_a * math_b;
    let division: i32 = math_a / math_b;
    let remainder: i32 = math_a % math_b;
    
    println!("{}", addition);
    println!("{}", subtraction);
    println!("{}", multiplication);
    println!("{}", division);
    println!("{}", remainder);
    
    // tuple
    println!("------------------------------ tuple ------------------------------");
    let tup: (i32, &str, bool) = (500, "hi", true);
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
    
    let (tup_x, tup_y, tup_z) = tup;
    println!("{}", tup_x);
    println!("{}", tup_y);
    println!("{}", tup_z);
}
