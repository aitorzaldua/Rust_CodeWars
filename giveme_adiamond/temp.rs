    /* match n {
        1 => Some ("*\n".to_string()),
        3 => Some (" *\n***\n *\n".to_string()),
        5 => Some ("  *\n ***\n*****\n ***\n  *\n".to_string()),
        _ => None
    } */


    let mut diamond = String::new();
    let mut diamond_temp = String::new();
    let mut space_str = String::new();

    if n % 2 !=0 {

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

    }

    Some (diamond_temp.to_string())
