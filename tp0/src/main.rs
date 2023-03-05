// TODO 3: Ecrivez la solution d'exercice 3 ici
fn divisible(n: u16, number:u16) -> bool{
    number % n == 0
}
// TODO 5: Ecrivez la solution d'exercice 5 ici
struct Computer {
    brand: String,
    processor: String,
    memory_size: u16
}

impl Computer {

    fn new(brand: String, processor: String, memory_size: u16) -> Computer {
        Computer{
            brand: brand,
            processor: processor,
            memory_size: memory_size
        }
    }

    fn display(&self) {
        println!("Computer details: {0} {1} {2}", self.brand, self.processor, self.memory_size);
    }
}
// TODO 6: Ecrivez la solution d'exercice 6 ici
fn menu(computers: &[Computer]) {
    let mut line = String::new();
   println!("Please enter a for printing all computers or b for printing the coputer with most memory:");

   std::io::stdin().read_line(&mut line);
   line.pop();
   let a: String = String::from('a');
   let b: String = String::from('b');

   while line.eq(&a) || line.eq(&b) {
        if line.eq(&a) {
            for i in computers {
                i.display();
            }
        } else if line.eq(&b) {
            let mut max2: u16 = 0;
            for i in computers {
                if i.memory_size > max2 {
                    max2 =i.memory_size;
                }
            }

            for i in computers {
                if i.memory_size == max2 {
                   println!("The computer with most memory is {0}",i.brand);
                }
            }

        }
        println!("Please enter a for printing all computers or b for printing the coputer with most memory:");
        line = String::new();
        std::io::stdin().read_line(&mut line);
        line.pop();
   }
}
fn main() {

    // TODO 1: Ecrivez la solution d'exercice 1 ici
    println!("Cristiana");

    // TODO 2: Ecrivez la solution d'exercice 2 ici
    let a: u16 = 4;
    let b: u16 = 3;
    println!("Max is {0}", if a > b {a} else {b});

    // TODO 4: Ecrivez la solution d'exercice 4 ici
    let a = [1, 2, 3, 4, 5];
    let mut max = a[0];
    for x in a {
        if x > max {
            max = x;
        }
    }
    println!("Maximum in array is {0}", max);

    println!("{0} is divisible by {1} is {2}", 25, 5, divisible(5, 25));

   // Test pour l'exercice 5
   let computer = Computer::new(String::from("Asus"), String::from("Intel"), 256);
   computer.display();
   
   // Test pour l'exercice 6
   let computers: [Computer; 2] = [computer, Computer::new(String::from("Apple"), String::from("M2"), 512)];   
   menu(&computers);
}
