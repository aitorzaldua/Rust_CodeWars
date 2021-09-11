/* What is an anagram? Well, two words are anagrams of each other if they both contain the same letters. For example:

'abba' & 'baab' == true

'abba' & 'bbaa' == true

'abba' & 'abbba' == false

'abba' & 'abca' == false
Write a function that will find all the anagrams of a word from a list. You will be given two inputs a word and an array with words. You should return an array of all the anagrams or an empty array if there are none. For example:

anagrams('abba', ['aabb', 'abcd', 'bbaa', 'dada']) => ['aabb', 'bbaa']

anagrams('racer', ['crazer', 'carer', 'racar', 'caers', 'racer']) => ['carer', 'racer']

anagrams('laser', ['lazing', 'lazy',  'lacer']) => [] */


fn main() {

    let input1 ="abba";
    let input2 = ["baba".to_string(), "abcd".to_string(), "baab".to_string(), "cccc".to_string()];

    let result = anagrams(input1, &input2);

    println!("{:?}", result);


}

fn anagrams(word: &str, words: &[String]) -> Vec<String> {

    let word_arr: Vec<char> = word.chars().collect();

    for i in words {
        for n in &word_arr {
            let s = i.contains(*n);
            if s {
                *i = str::replace(&i, *n, "");
            }

            println!("{} in  {}? => {}", n, i, s);
         }

    }



    let end = vec!["hello".to_string(), "world".to_string()];

    end
}

