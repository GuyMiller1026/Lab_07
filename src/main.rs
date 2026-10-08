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
    let mut log: Vec<Reading> = Vec::new();
    log.push(Reading { day: DAYS[0].to_string(), high: 72 });
    log.push(Reading { day: DAYS[1].to_string(), high: 68 });
    log.push(Reading { day: DAYS[2].to_string(), high: 75 });
    log.push(Reading { day: DAYS[3].to_string(), high: 81 });
    log.push(Reading { day: DAYS[4].to_string(), high: 79 });
    for i in 0..log.len() {
        println!("{}: {}", log[i].day, log[i].high);
    }

    let mut rng = rand::rng();
    let day6: i32 = rng.random_range(60..=100);
    let day7: i32 = rng.random_range(60..=100);
    log.push(Reading { day: DAYS[5].to_string(), high: day6 });
    log.push(Reading { day: DAYS[6].to_string(), high: day7 });

    println!("Full week:");
    for i in 0..log.len() {
        println!("{}: {}", log[i].day, log[i].high);
    }
    let summary = summarize(&log, 75);
    println!("Average: {}", summary.average);
    println!("Hottest day: {}", summary.hottest_day);
    println!("Days above 75: {}", summary.days_above);
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

fn hottest_day(log: &Vec<Reading>) -> usize {
    let mut hottest = 0;
    for i in 0..log.len() {
        if log[i].high > log[hottest].high {
            hottest = i;
        }
    }
    hottest
}
fn count_above(log: &Vec<Reading>, threshold: i32) -> usize {
    let mut count = 0;
    for i in 0..log.len() {
        if log[i].high > threshold {
            count = count + 1;
        }
    }
    count
}
fn summarize(log: &Vec<Reading>, threshold: i32) -> Summary {
    Summary {
        average: average_temp(log),
        hottest_day: log[hottest_day(log)].day.clone(),
        days_above: count_above(log, threshold),
    }
}
