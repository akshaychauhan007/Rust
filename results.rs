// Result in rust is used similarly like try and catch in javascript . Here we have Result enum which has two variants OK and Err. In below code we read file if file is found we pattern match that with the result if file doesnt exist or some kind of error occurs during reading the file it returns Err variant if everything works well it rerturns OK variant 

use std::fs::read_to_string; //here we are importing read_to_string function from fs library as it is not builtin globally to use

fn main() {
    let ans = read_files_from_system(String::from("a.txt"));
}

fn read_files_from_system(file_path: String) -> Result<String, String> {
    let result = read_to_string(file_path); // Result
    match result {
        Ok(data) => Ok(data),
        Err(err) => Err(String::from("File not read")),
    }
}