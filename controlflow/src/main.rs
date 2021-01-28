fn five() -> i32 {
    5
}
fn six() -> i32{
    8
    
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
    let z = six();
   if z > 10 {

       println!("z is greater");
   }else {
     println!("z is small");
   }
   

}
