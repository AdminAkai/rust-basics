fn main() {
    let one: i32 = 1;
    if_conditions(one);
    looping();
    while_looping();
    while_looping_with_vector();
}

fn if_conditions(one: i32) {
        if one > 0 {
            println!("True")
        } else if one == one {
            println!("Equal")
        } else {
            println!("False")
        }
}

fn looping() {
    let mut num = 0;

    'counter: loop {
        println!("Count: {}", num);
        let mut decrease = 5;
        loop {
            println!("Decreasing: {}", decrease);
            if decrease == 4 {
                break
            }
            if num == 2 {
                break 'counter
            }
            decrease -= 1;
        }
        num += 1;
    }
}

fn while_looping() {
    let mut num = 0;
    while num < 5 {
        println!("Num: {}", num);
        num += 1;
    }
}

fn while_looping_with_vector() {
    let vec: Vec<i8> = (0..10).collect();

    for element in vec {
        println!("{}", element);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }
    
    println!("Liftoff!");
}