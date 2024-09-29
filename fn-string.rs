//======================function to calculate length of a string=====================

fn main(){
    let name =String::from("akshay");
    let len = len_of_str(name);
    println!("{} is the length of string",len)

}

fn len_of_str(str:String)->usize{
    str.chars().count() //here u dont need to type "return" it automatically knows
}