/* A man has a rather old car being worth $2000. He saw a secondhand car being worth $8000. He wants to keep his old car until he can buy the secondhand one.

He thinks he can save $1000 each month but the prices of his old car and of the new one decrease of 1.5 percent per month. Furthermore this percent of loss increases of 0.5 percent at the end of every two months. Our man finds it difficult to make all these calculations.

Can you help him?

How many months will it take him to save up enough money to buy the car he wants, and how much money will he have left over? */

//cómo retornar 2 valores - tipos de numeros - redondear un float 2 formas

fn main() {

    let old: i32 = 8000;
    let new: i32 = 8000;
    let saving: i32 = 500;
    let perc: f64 = 1.0;
    let result = nb_months(old, new, saving, perc);
    println!("{:?}", result);
}

fn nb_months(old: i32, new: i32, saving: i32, mut perc: f64) -> (i32, i32) {
    let mut counter_months = 0;
    let mut saving_f: f64 = 0.0;
    let monthly_f = saving as f64;
    let mut new_f = new as f64;
    let mut old_f = old as f64;

    loop {
        counter_months += 1;

        if counter_months % 2 == 0 {
            perc = perc + 0.5;
        }

        if saving_f + old_f - new_f >= 0.0 {
            break
        }

        saving_f = monthly_f + saving_f;
        old_f = old_f - (old_f * perc * 0.01);
        new_f = new_f - (new_f * perc * 0.01);

    }

    let mut available = saving_f + old_f  - new_f;
    available = (available.round() as i32).into();


    (counter_months -1, available as i32)

}

