// Write a function which takes a number as an input and return true or false if even or odd

fn main(){
    println!("{}",is_even(9))

}

fn is_even(num:i32)->bool{

    if num % 2 == 0 {
        return true
    }
    return false
}