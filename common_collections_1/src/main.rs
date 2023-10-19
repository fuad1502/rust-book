use std::collections::HashMap;

fn main() {
    // input
    let mut integers = [
        2, 3, 5, 1, 4, 6, 2, 5, 7, 2, 5, 3, 2, 4, 2, 3, 4, 1, 2, 5, 1, 2, 4, 1,
    ];

    // sorting method for finding median and mode
    integers.sort();
    let median = integers[integers.len() / 2];
    let mut max_frequency = 0;
    let mut mode = integers[0];

    let mut last_value = integers[0];
    let mut frequency = 0;
    for i in integers {
        if i == last_value {
            frequency += 1;
        } else {
            frequency = 1;
        }
        if frequency > max_frequency {
            max_frequency = frequency;
            mode = i;
        }
        last_value = i;
    } 

    println!("Sorting method:");
    println!("Integers = {:?}", integers);
    println!("Median = {median}");
    println!("Mode = {mode}");
    println!("");

    // HashMap method for finding mode
    let mut max_frequency = 0;
    let mut mode = integers[0];
    let mut frequencies: HashMap<i32, i32> = HashMap::new();

    for i in integers {
        let entry = frequencies.entry(i).or_insert(0);
        *entry += 1;
        if *entry > max_frequency {
            max_frequency = *entry;
            mode = i;
        }
    }

    println!("HashMap method:");
    println!("HashMap = {:?}", frequencies);
    println!("Mode = {mode}");
}
