const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];
	for i in 0..highs.len() {
        	println!("{}: {}", DAYS[i], highs[i]);
    }
}

