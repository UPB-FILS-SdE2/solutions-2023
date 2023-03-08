use std::env;

// Ex 4
fn division(a: i32, b:i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a/b)
    }
}

fn remainder(a: i32, b:i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a%b)
    }
}

fn mul(a: i32, b:i32) -> i32 {
    a * b
}

fn add(a: i32, b:i32) -> i32 {
    a + b
}

fn sub(a: i32, b:i32) -> i32 {
    a - b
}

fn main() {
    // example d'utilisation: CMD=add cargo run 5 3
    
    let args: Vec<String> = env::args().collect();

    let comm = match env::var("CMD") {
            Ok(s) => s,
            Err(_) => {
                println!("Comanda nu este valida.");
                std::process::exit(-1);
            }
        };

        let a = match args[1].parse::<i32>() {
            Ok(result) => result,
            Err(_) => std::process::exit(-1),
        };

        let b = match args[2].parse::<i32>() {
            Ok(result) => result,
            Err(_) => std::process::exit(-1),
        };
  
        match comm.as_str() {
            "add" => println!("Result of add operation {0}", add(a, b)),
            "sub" => println!("Result of sub operation {0}", sub(a, b)),
            "mul" => println!("Result of mul operation {0}", mul(a, b)),
            "div" => {
                let div_res = division(a, b);
                match div_res {
                    Some(result) => println!("Result of div is {0}", result),
                    None => println!("Division by 0 occured")
                }
            },
            "rem" => {
                let rem_res = remainder(a, b);
                match rem_res {
                    Some(result) => println!("Result of rem is {0}", result),
                    None => println!("Rem by 0 occured")
                }
            },
            _ => println!("Please enter a valid command!")
        } 
  }

