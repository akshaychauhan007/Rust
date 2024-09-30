
// Options are used to handel null similar to js here it gives us option which return "some" in case condition is met else "None" if there value is not found. It is used to reduce the null/undefined exceptions that are caused in JS

fn main() {
    let index = find_first_a(String::from("john"));

    match index {
        Some(value) => println!("index is {}", value),
        None => println!("a not found"),
    }
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }

    return None;
}