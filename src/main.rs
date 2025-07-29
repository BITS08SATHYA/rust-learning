// #[allow(unused_variables)]
// const TOUCHDOWN_POINTS:i32 = 6;
// const TAX_RATE:f64 = 7.25;
//
// type Meters = i32;

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

    // let season = "FALL";
    //
    // let mut points_scored = 28;
    // points_scored = 35;
    //
    // let mut event_time = "06:00";
    // let event_time: i32 = 6;
    //
    // println!("My Favorite season is {} , the points scored {} and the current event time is {} "
    //          , season, points_scored, event_time);
    //
    // let favorite_beverage = "whisky";


    //     Data types
    //     Integer
    //     let eight_bit:u8 = 255;
    //
    //
    //     println!("eight_bit: {}", eight_bit);

    //     Strings and Raw Strings
    //     println!("Hello String! \"I love you!\" ");
    //
    //     let filepath = "C:\\My Documents\\new\\videos";
    //     println!("{}" , filepath);
    //
    //     let value = -15i16;
    //     println!("{}", value.abs());
    //
    //     let empty_space = "     my content     ";
    //     println!("{}" , empty_space.trim());
    //
    //     println!("{}", value.pow(2));
    //     println!("{}", value.pow(3));

    //     floats
    //     let pi = 3.14159;
    //     let pi = 3.14159265358979322384f64;
    //     println!("The current value of pi is: {:.4}", pi);

    // println!("{}", pi.floor());
    // println!("{}", pi.ceil());
    // println!("{}", pi.round());

    //      Casting as keyword
    //     let miles_away = 50;
    //     let miles_away_i8 = miles_away as u8;
    //     let miles_away_int = miles_away as i32;
    //
    //     println!("Miles away: {}", miles_away_int);

    //     Math Operations
    //     let addition = 5 + 4;
    //     let subtraction = 10  - 6;
    //     let multiplication = 3 * 4;
    //
    //     println!("Addition: {addition}, subtraction: {subtraction}, multiplication: {multiplication}");
    //
    //     let floor_division = 5 / 3;
    //     println!("{floor_division}");
    //
    //     let decimal_division = 5.0/3.0;
    //     println!("{decimal_division:.2}");
    //
    //     let remainder = 8 % 2;
    //     println!("{remainder}");

    // let mut year = 2025;
    // year += 1;
    // println!("The new year is {year}");

    // println!("{}", !true);
    // println!("{}", !false);
    //
    // let age = 13;
    // let can_see_rate_r_movie = age >= 17;
    // println!("I am {age} years old, Can i not see this scary movie? {can_see_rate_r_movie} ");
    //
    //
    // let employee = ("Molly", 34, "Marketing");
    //
    // let (name, age, department) = employee;
    //
    // println!("Name: {name}, Age: {age}, Department: {department}");
    //
    // println!("{employee:#?}");

    // let month_days = 1..31;
    //
    // let month_days = 1..=31;
    // println!("Month days: {month_days:?}");
    //
    // for number in month_days {
    //     println!("{number}")
    // }
    //
    // let letters = 'a'..='z';
    //
    // for letter in letters {
    //     println!("{letter}");
    // }
    //
    // let colors = ["Red", "Green", "Blue"];
    //
    // for color in colors {
    //     println!("{color} is a great color!")
    // }

    //     open_store("Newport");
    //     bake_pizze();
    //     swim_in_profit();
    //
    //     let result = square(8);
    //     println!("{result}");
    //
    //     let multiplier = 3;
    //
    //     let calculation = {
    //         let value = 5 + 4;
    //         value * multiplier
    //     };
    //
    //     println!("{calculation}")
    // }
    //
    // fn square(number: i32) -> i32 {
    //     number * number
    // }
    //
    // fn mystery(){
    //     println!("Hello there!");
    // }
    //
    // fn open_store(neighborhood: &str){
    //     println!("Opening my pizza store {neighborhood}");
    // }
    //
    // fn bake_pizze(){
    //     println!("Bake my pizza store");
    // }
    //
    // fn swim_in_profit(){
    //     println!("SO much $$$, so litte time");
    // }

    // let season = "summer";
    //
    // if season == "summer" {
    //     println!("School's out");
    // }else if season == "winter" {
    //     println!("School's closed")
    // }

    // let evaluation = true;
    //
    // let value = match evaluation {
    //
    //     true => {
    //         println!("The value is true");
    //     }
    //     false => {
    //         println!("The value is false");
    //     }
    //
    // };

    // println!("{value}");

    // let number = 9;

    // match number {
    //     2 | 4 | 6 | 8 => println!("{number} as even"),
    //     1 | 3 | 5 => println!("{number} as odd"),
    //     _ => println!("unknown for me"),
    // }

    // match number {
    //     value  if value % 2 == 0 => println!("{value} is an even number"),
    //     x if x % 2 == 0 => println!("{x} is an odd number"),
    //     _ => println!("Unknown number"),
    // }

    // let mut seconds = 21;
    //
    // while seconds > 0 {
    //
    //     // if seconds == 0{
    //     //     println!("Blastoff!");
    //     //     break;
    //     // }
    //
    //     if seconds % 2 == 0{
    //         println!("{seconds} seconds (even number), skipping 3 seconds..");
    //         seconds -= 3;
    //         continue;
    //     }
    //
    //     println!("{seconds} seconds to blastoff...");
    //     seconds -= 1
    // }

    // countdown(5);

    // let age = 33;
    //
    // {
    //     let is_handsome = true;
    // }
    //
    // println!("{age}")


    // let time = 2025;
    // let year = time;
    //
    // println!("The time is {time}. It is the year {year}")

    let text = String::new();
    let candy = String::from("Kitkat");
    // println!("{candy}");
    
    let mut name = String::from("Boris");
    // println!("{name}");

    name.push_str(" Pask");
    println!("{name}");
    
    let person = String::from("Boris");
    let genius = person.clone();

    println!("This is {person}");
    
}

// fn countdown(seconds: i32){
//
//     if seconds == 0{
//         println!("Blastoff");
//     }else{
//         println!("{} seconds to blastoff...", seconds);
//         countdown(seconds - 1);
//     }
// }


// fn even_or_odd(number: i32) {
//     if number % 2 == 0{
//         "even";
//     }else{
//         "odd";
//     }
//
//     let result = if number % 2 == 0 {"even"} else {"odd"};
//     println!("The numbe is {result}");
// }