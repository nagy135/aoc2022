use std::fs;

fn main() {
    let result = day1();
    println!("result: {}", result);
}

fn day1() -> u32 {
    let contents = fs::read_to_string("assets/real/day1").expect("Missing file");

    let mut buckets: Vec<u32> = vec![];
    let mut current_bucket: u32 = 0;
    for line in contents.lines() {
        match line.is_empty() {
            true => {
                buckets.push(current_bucket);
                current_bucket = 0;
            }
            false => {
                current_bucket += line.parse::<u32>().expect("Line cant be parsed");
            }
        }
    }
    buckets
        .iter()
        .max()
        .expect("Nothing returned as max value")
        .to_owned()
}
