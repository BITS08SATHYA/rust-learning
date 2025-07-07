#[allow(unused_variables)]
const TOUCHDOWN_POINTS:i32 = 6;
const TAX_RATE:f64 = 7.25;

type Meters = i32;

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
//     let grams_of_protein = "100.345";
//     let grams_of_protein = 100.345;
//     let grams_of_protein = 100;
//
//     let income = 100000;
//     println!("THe tax rate is {income} and my tax rate is {TAX_RATE}")
//
//     let mile_race_length: Meters = 1600;
//     let two_mile_race_length: Meters = 3200;
//     println!("A one mile race is {mile_race_length} and two_mile_race_length is {two_mile_race_length}");

    let season = "FALL";

    let mut points_scored = 28;
    points_scored = 35;
    
    let mut event_time = "06:00";
    let event_time: i32 = 6;
    
    println!("My Favorite season is {} , the points scored {} and the current event time is {} "
             , season, points_scored, event_time);
    
    let favorite_beverage = "whisky";

}
