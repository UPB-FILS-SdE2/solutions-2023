use std::env;

// Ex 2
fn division(a: i32, b:i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a/b)
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        std::process::exit(-1)
    } else {
        let a = args[1].parse::<i32>();
        let b = args[2].parse::<i32>();
        let res = division(a.unwrap(), b.unwrap());

        match res {
            Some(result) => println!("The result is: {0}", result),
            None => println!("The case of 0 division")
        } 
  }
}
