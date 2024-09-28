fn main() {
    let num: i32 = 11;
    println!("is number even : {}",is_even(num));
}

//function iseven

fn is_even(num:i32)->bool{
    if num % 2 ==0 {
        return true;
        
    }
    return false;
}