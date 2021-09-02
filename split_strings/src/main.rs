/* Complete the solution so that it splits the string into pairs of two characters. If the string contains an odd number of characters then it should replace the missing second character of the final pair with an underscore ('_').

Examples:

solution("abcdef") // should return ["ab", "cd", "ef"]
solution("abcdefg") // should return ["ab", "cd", "ef", "g_"]
 */


fn main() {

    let s = String::from("abcdefg");
    let z = String::from("Hello_world");

    let result_01 = solution_1(&s);
    let result_02 = solution_2(&z);

    println!("{:?}", result_01);
    println!("{:?}", result_02);


}

fn solution_1(s: &str) -> Vec<String> {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|v| {
            if v.len() < 3 {
                format!("{}_", v[0])
            } else {
                v.into_iter().collect()
            }
        })
        .collect()
}

fn solution_2(s: &str) -> Vec<String> {
    s.chars()
        //.chain(std::iter::once(' _'))
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

