fn main() {

    // The return value of the function is an option
let result = print(1);

println!("*\n***\n *\n");
}


fn print(n: i32) -> Option<String> {
    if n % 2 == 0 && n <= 0  {
        None
    } else if n == 1 {
        Some (" *\n***\n *\n".to_string()) 
    }
    else {
        Some (n.to_string())    }
}