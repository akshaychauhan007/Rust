//=========================Structs/Similar to Objects in JS===================
struct Customer {
    first_name: String,
    last_name:String,
    age:u32
}

fn main(){
    let customer1 = Customer{
        first_name:String::from("Majnu"),
        last_name:String::from("Khan"),
        age:66
    };
    println!("{}",customer1.first_name)
}