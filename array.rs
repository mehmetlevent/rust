fn main(){
    // arrays 1, 2, 3, 4, 5
   // let numbers = [1, 2, 3, 4, 5];
    let numbers = [2; 200];
    for i in 0..numbers.len(){
    //for i in numbers.iter(){
        println!("{}", numbers[i]);
    }
}
