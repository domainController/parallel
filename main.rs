fn main (){
    for x in 1..101 {
        if x % 15 == 00 {
            println!("FizzBuzz");
        }   
        else if x % 3 == 00 {
            println!("Buzz");
        }
        else if x % 5 == 00 {
            println!("Fizz");
        }
        else {
            println!("{}", x);
        }
    }
}
        

