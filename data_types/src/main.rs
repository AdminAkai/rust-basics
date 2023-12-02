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

    // char
    println!("------------------------------ char ------------------------------");
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
    
    // arrays
    println!("------------------------------ arrays ------------------------------");
    let array = [1, 2, 3];    
    println!("{}", array[0]);
    
    let mut array2: [i32; 3] = [4, 5, 6];
    println!("{}", array2[0]);
    array2[0] = 10;
    println!("{}", array2[0]);
    
    // vectors
    println!("------------------------------ vectors ------------------------------");
    let mut nums = vec![1, 2, 3];    
    
    nums.push(4);
    println!("{:?}", nums);
    
    nums.pop();
    println!("{:?}", nums);
    
    let mut vec = Vec::new(); // calling vec! constructor
    vec.push("test");
    vec.push("string");
    println!("{:?}", vec);
    
    vec.reverse();
    println!("{:?}", vec);
    
    let vec_two: Vec<i32> = Vec::<i32>::with_capacity(2);
    println!("{:?}", vec_two.capacity());
    
    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);
    
    let slice_v: &[i32] = &v[2..4];
    println!("{:?}", slice_v);
    
    // Strings
    println!("------------------------------ Strings ------------------------------");
    
    let name: String = String::from("Test");
    let course: String = "Rust".to_string();
    let new_name: String = name.replace(&name, "Test 2");
    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name);
    
    let str_one = "hello";
    let str_two = str_one.to_string();
    let str_three = &str_two;
    println!("{}", str_one);
    println!("{}", str_two);
    println!("{}", str_three);
    
    let rust = "\x52\x75\x73\x74";
    println!("{}", rust);
}
