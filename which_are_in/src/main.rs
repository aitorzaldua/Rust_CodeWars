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

    let a1 = vec!["xyz", "live", "strong", "ali"];
    let a2 = vec!["lively", "alive", "harp", "sharp", "armstrong"];


    fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
        let mut result:Vec<String> = Vec::new();
        for i in arr_a {
            for x in arr_b {
                if x.contains(i) {
                    if result.contains(&i.to_string()) {
                        break
                    }
                    else {
                        result.push(i.to_string());

                    }
                }
            }
        }

        result.sort();
        result.dedup();
        result
        


    }

    println!("{:?}", in_array(&a1, &a2))   ;


}
