use std::io;
use std::cmp::Ordering;

fn main() {
    fibonacci()

}
fn fibonacci() {
    let mut fib0 = 0;
    let mut fib1 = 1;
    let mut fib = 0;

    for i in (0..25){
        if i == fib0{
            println!("{}", fib0);
        }

        else if i == fib1{
            println!("{}", fib1);
        }

        else {
            fib = fib0 + fib1;
            println!("{} {}", fib, fib1);
            fib0 = fib1;
            fib1 = fib;




        }
    }


}
fn convert_farengeit_to_c(){
    println!("farengeit to c");

    loop {
        let mut farengeit = String::new();
        io::stdin().read_line(&mut farengeit)
            .expect("Failed to read line");

        let farengeit: f64 = match farengeit.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut celsium = (farengeit - 32.0) * (5.0/9.0);

        println!("reult:{celsium}");
    }
}

fn test_1() {
    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;

    }

}

fn test_function() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaing = 10;

        loop{
            println!("remaing = {remaing}");
            if remaing == 9 {
                break;

            }
            if count == 2 {
                break 'counting_up;
            }
            remaing -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn gg() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}