fn division(a: i32, b:i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a/b)
    }
}

fn main() {
    let a = 10;
    let b = 2;
    let res = division(a, b);

    match res {
        Some(x) => println!("The result is {}", x),
        None => println!("Case of division by 0") 
    }
}
