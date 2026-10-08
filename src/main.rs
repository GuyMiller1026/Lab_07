use rand::Rng;
const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
struct Reading {
    day: String,
    high: i32,
}

struct Summary {
    average: f64,
    hottest_day: String,
    days_above: usize,
}
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
    let mut log: Vec<Reading> = Vec::new();
    for i in 0..highs.len() {
        log.push(Reading {
            day: DAYS[i].to_string(),
            high: highs[i],
        });
    }

    println!("Full week:");
    for i in 0..highs.len() {
        println!("{}: {}", DAYS[i], highs[i]);
    }
    let avg = average_temp(&log);
    println!("Average: {}", avg);

    let hot = hottest_day(&highs);
    println!("Hottest day: {}", DAYS[hot]);

    let above = count_above(&highs, 75);
    println!("Days above 75: {}", above);
}

    fn average_temp(log: &Vec<Reading>) -> f64 {
    if log.len() == 0 {
        return 0.0;
    }
    let mut total = 0;
    for i in 0..log.len() {
        total = total + log[i].high;
    }
    total as f64 / log.len() as f64
}
fn hottest_day(log: &Vec<i32>) -> usize {
    let mut hottest = 0;
    for i in 0..log.len() {
        if log[i] > log[hottest] {
            hottest = i;
        }
    }
    hottest
}

fn count_above(log: &Vec<i32>, threshold: i32) -> usize {
    let mut count = 0;
    for i in 0..log.len() {
        if log[i] > threshold {
            count = count + 1;
        }
    }
    count
}
