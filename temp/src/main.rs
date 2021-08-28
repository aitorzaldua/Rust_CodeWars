fn main() {

    let charachter = "a";
    let string_1 = String::from("hello");
    let number = 5;

    let s = vec![1, 7, 4, 5];
    let z = vec!["yes", "no", "a"];
    let y = vec!["yes", "no", "hello"];

    //1.- 
    let answer_1 = s.contains(&number);

    println!("s contains number? => {:?}", answer_1);

    //2.- 
    let answer_3 = z.contains(&charachter);

    println!("z contains charachter? => {:?}", answer_3);

    //3.- Contains doesn´t works with strings:
    let answer_2 = z.into_iter().any(|e| e == &string_1);
    let answer_4 = y.into_iter().any(|e| e == &string_1);
    println!("z contains string_1? => {:?}", answer_2);
    println!("y contains string_1? => {:?}", answer_4);



}
