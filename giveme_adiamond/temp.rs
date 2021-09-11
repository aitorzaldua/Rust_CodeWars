    /* match n {
        1 => Some ("*\n".to_string()),
        3 => Some (" *\n***\n *\n".to_string()),
        5 => Some ("  *\n ***\n*****\n ***\n  *\n".to_string()),
        _ => None
    } */


   

    let x = (1..=n)
        .chain((1..n).rev())
        .step_by(2)
        .map(|i| format!("{}{}", " ".repeat((n - i) / 2), "Hello".repeat(i)))
        .collect::<String>();


        let n = 20;
        let x = (1..=n)
        .chain((1..n).rev())
        .step_by(2)
        .collect::<Vec<i32>>();
    println!("{:?}", x);


    let x = (1..=n)
    .chain((1..n).rev())
    .collect::<Vec<i32>>();


    let x = (1..=10)
    .collect::<Vec<i32>>();