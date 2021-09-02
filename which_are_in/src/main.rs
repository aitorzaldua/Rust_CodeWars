/* Given two arrays of strings a1 and a2 return a sorted array r in lexicographical order of the strings of a1 which are substrings of strings of a2.

Example 1:
a1 = ["arp", "live", "strong"]
a2 = ["lively", "alive", "harp", "sharp", "armstrong"]
returns ["arp", "live", "strong"]

Example 2:
a1 = ["tarp", "mice", "bull"]

a2 = ["lively", "alive", "harp", "sharp", "armstrong"]

returns [] */


fn main() {


    let arr_a = ["arp", "live", "strong"];
    let arr_b = vec!["lively", "alive", "harp", "sharp", "armstrong"];

    let result = in_array(&arr_a, &arr_b);

    println!("{:?}", result);



}

fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut r = arr_a
        .into_iter()
        .filter(|&s| arr_b.iter().any(|&bs| bs.contains(s)))
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    r.sort();
    r.dedup();
    r
}