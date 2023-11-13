fn main() {
    for i in 1u32..100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        }else if i % 5 == 0 {
            println!("Buzz");
        }
        println!("{}", i);
    }
}