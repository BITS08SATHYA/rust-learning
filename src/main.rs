const TAX_RATE:f64 = 7.25;

fn main() {

    // let apples = 50;
    // let oranges = 14 + 6;
    // let fruits = apples + oranges;
    //
    // println!("Fruits count is: {apples} and {oranges} is {fruits}");

    // let mut gym_reps = 15;
    //
    // println!("I plan to do {gym_reps} reps");
    //
    // gym_reps = 25;
    //
    // println!("I now plan to do {gym_reps} reps");

//     Variable shadowing
    let grams_of_protein = "100.345";
    let grams_of_protein = 100.345;
    let grams_of_protein = 100;

    let income = 100000;
    println!("THe tax rate is {income} and my tax rate is {TAX_RATE}")
}
