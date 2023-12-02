fn main() {
    print_phase("Print my argunment");
    println!("{}", gcd(20, 5));
    println!("{}", multiple_return_values(true));
    println!("{}", multiple_return_values(false));

    let hello: String = String::from("Hello");
    println!("{}",concat_string(hello));
}

fn print_phase(phrase: &str) {
    println!("{}", phrase);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b
    }
    b
}

fn multiple_return_values(flag: bool) -> bool {
    if flag == true {
        true
    } else {
        false
    }
}

fn concat_string(string_arg: String) -> String {
    [string_arg, " World".to_string()].concat()
}