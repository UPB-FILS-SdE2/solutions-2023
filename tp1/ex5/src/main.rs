use std::env;

// Ex 5
fn division(arr: &Vec<i32>) -> Option<i32> {
    let mut result = arr[0];
    if arr.len() < 2 {
        None
    } else {
        for i in 0..arr.len() {
            result = result/arr[i];
        }
        Some(result)
    }
    
}

fn remainder(arr: &Vec<i32>) -> Option<i32> {
    let mut result = arr[0];
    if arr.len() < 2 {
        None
    } else {
        for i in 0..arr.len() {
            result = result%arr[i];
        }
        Some(result)
    }
}

fn mul(arr: &Vec<i32>) -> i32 {
    let mut result = 1;
    for i in 0..arr.len() {
        result = result * arr[i];
    }
    result
}

fn add(arr: &Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..arr.len() {
        result = result + arr[i];
    }
    result
}

fn sub(arr: &Vec<i32>) -> i32 {
    let mut result = arr[0];
    for i in 1..arr.len() {
        result = result - arr[i];
    }
    result
}

fn avg(arr: &Vec<i32>) -> f32 {
    let mut result = 0;
    for i in 0..arr.len() {
        result = result + arr[i];
    }
    (result/(arr.len() as i32)) as f32
}

fn sort_custom(arr: &mut Vec<i32>) {
    arr.sort()
}

fn unique(arr: &mut Vec<i32>) {
    arr.dedup()
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let comm = &args[1];

    let mut arr = Vec::new();
    for i in 2..args.len() {
        arr.push(match args[i].parse::<i32>() {
                Ok(result) => result,
                Err(_) => std::process::exit(-1),
        });
    }
  
    match comm.as_str() {
        "add" => println!("Result of add operation {0}", add(&arr)),
        "sub" => println!("Result of sub operation {0}", sub(&arr)),
        "mul" => println!("Result of mul operation {0}", mul(&arr)),
        "div" => {
                let div_res = division(&arr);
                match div_res {
                    Some(result) => println!("Result of div is {0}", result),
                    None => println!("Division by 0 occured")
                }
            },
        "rem" => {
                let rem_res = remainder(&arr);
                match rem_res {
                    Some(result) => println!("Result of rem is {0}", result),
                    None => println!("Rem by 0 occured")
                }
            },
        "avg" => println!("Result of avg operation {0}", avg(&arr)),
        "sort" => {
            sort_custom(&mut arr);
            for i in 0..arr.len() {
                println!("Element at position {0} is now {1}", i, arr[i]);
            }
           },
        "unique" => {
            unique(&mut arr);
            for i in 0..arr.len() {
              println!("Element at position {0} is now {1}", i, arr[i]);
            }
           },
            _ => println!("Please enter a valid command!")
        }
  }

