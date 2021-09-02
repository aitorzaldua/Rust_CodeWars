/* 
You need to return a string that looks like a diamond shape when printed on the screen, using asterisk (*) characters. Trailing spaces should be removed, and every line must be terminated with a newline character (\n).
Return null/nil/None/... if the input is an even number or negative, as it is not possible to print a diamond of even or negative size.

Examples
A size 3 diamond:

 *
***
 *
...which would appear as a string of " *\n***\n *\n"

A size 5 diamond:

  *
 ***
*****
 ***
  *
...that is:

"  *\n ***\n*****\n ***\n  *\n" */




fn main() {
let result = print(5);

println!("{:?}", result);
}


fn print(n: i32) -> Option<String> {
    if n % 2 == 0 || n <= 0 {
        None
    }
    else {
        let mut diamond = String::new();
        let mut diamond_temp = String::new();
        let mut space_str = String::new();

        for x in 1 .. n+1 {
            if x % 2 != 0 {
                let diamond_vec = vec!["*"; x as usize];
                diamond = diamond_vec
                .into_iter()
                .collect();
                let formula = (n - x) / 2;
                let space_vec = vec![" "; formula as usize];
                space_str = space_vec
                .into_iter()
                .collect();
                diamond_temp.push_str(&space_str);
                diamond_temp.push_str(&diamond);
                diamond_temp.push_str("\n");
            }

        }

        for x in (1 .. n-1).rev()  {
            if x % 2 != 0 {
                let diamond_vec = vec!["*"; x as usize];
                diamond = diamond_vec
                .into_iter()
                .collect();
                let formula = (n - x) / 2;
                let space_vec = vec![" "; formula as usize];
                space_str = space_vec
                .into_iter()
                .collect();
                diamond_temp.push_str(&space_str); 
                diamond_temp.push_str(&diamond);
                diamond_temp.push_str("\n");
            }
    }
    Some (diamond_temp.to_string())
    }

}