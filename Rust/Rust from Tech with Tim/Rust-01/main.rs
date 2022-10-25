fn main() {
    let mut x = 4;
    println!("X is {}", x);
    x = 5;
    println!("X is {}", x);

    let x = 4;
    println!("X is {}", x);
    let x = 5;
    println!("X is {}", x);

    let x = 4;
    println!("X is {}", x);
    {
        let x = 2;
        println!("X is {}", x);
    }
    let x = x + 1;
    println!("X is {}", x);

    let x = 4;
    println!("X is {}", x);
    {
        let x = x - 1;
        println!("X is {}", x);
    }
    let x = x + 1;
    println!("X is {}", x);

    let x = 4;
    println!("X is {}", x);
    let x = "Hello World!!";
    println!("X is {}", x);

    const CONSTANT: u32 = 60;
    println!("{}", CONSTANT);
}
