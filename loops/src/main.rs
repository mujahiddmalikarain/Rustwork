fn main() {
    let mut count = 0;
    another();

 let result=loop {
     count +=1;
     if count == 10 {
        break count * 3;

     }

 };

    println!("The result is break loop result {}", result);
}


fn another() {
    for number in (1..100).rev() {
        println!(" for loop result{}!", number);
    }
    println!("LIFTOFF!!!");
}