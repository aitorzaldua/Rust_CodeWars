/* Complete the solution so that it splits the string into pairs of two characters. If the string contains an odd number of characters then it should replace the missing second character of the final pair with an underscore ('_').

Examples:

solution("abcdef") // should return ["ab", "cd", "ef"]
solution("abcdefg") // should return ["ab", "cd", "ef", "g_"]
 */


fn main() {

    let mut s = String::from("abcdefg");

    if s.len() % 2 != 0 {
        s.push_str("_");
    }

    let temp_string = String::new();

    fn working (s: String, mut temp_string: String) -> Vec<String>{
        let mut output: Vec<String> = vec![];
        for i in s.chars(){
            if temp_string.len() != 2 {
                temp_string.push(i);

            }
            else {
                output.push(temp_string.to_string());
                temp_string.clear();
                temp_string.push(i);
            }
        }
        
        output.push(temp_string.to_string());

        output

    }

    let mut result = working(s, temp_string);

    if result[0] == "" {
        result = vec![];
    }

    println!("{:?}", result);


}

