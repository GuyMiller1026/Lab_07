use rand::Rng;
const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];
	for i in 0..highs.len() {
        	println!("{}: {}", DAYS[i], highs[i]);
    }

    let mut rng = rand::rng();
    let day6: i32 = rng.random_range(60..=100);
    let day7: i32 = rng.random_range(60..=100);
    highs.push(day6);
    highs.push(day7);


    println!("Full week:");
    for i in 0..highs.len() {
        println!("{}: {}", DAYS[i], highs[i]);
    }

}

