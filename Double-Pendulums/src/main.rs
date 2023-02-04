fn main() {
    for n in 1..101 {
        fizz_buzz(&n)
    }
}

fn fizz_buzz(n: &i32) {
    let mut divisibility = false;
    let fizz_number = 3;
    let buzz_number = 5;

    if n % fizz_number == 0 {
        print!("Fizz");
        divisibility = true
    }

    if n % buzz_number == 0 {
        print!("Buzz");
        divisibility = true
    }

    if divisibility == false {
        print!("{}", n);
    }

    println!("")
}
