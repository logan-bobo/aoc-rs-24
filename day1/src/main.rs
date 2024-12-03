use std::fs::read_to_string;

fn main() {
    let data = read_to_string("./input.txt").expect("Input file is not at expected location");

    let mut distance: usize = 0;
    let mut left_locations: Vec<usize> = Vec::new();
    let mut right_locations: Vec<usize> = Vec::new();

    for item in data.split("\n") {
        if item.len() == 0 {
            break;
        }

        let split_locations: Vec<usize> = item
            // left and right location in space is separated by triple space
            .split("   ")
            .map(|item| item.parse::<usize>().unwrap())
            .collect();

        // We assume every location pair is always populated
        left_locations.push(split_locations[0]);
        right_locations.push(split_locations[1]);
    }

    if left_locations.len() != right_locations.len() {
        panic!("locations do not mathch")
    }

    left_locations.sort();
    right_locations.sort();

    for (index, element) in left_locations.iter().enumerate() {
        match element.cmp(&right_locations[index]) {
            std::cmp::Ordering::Less => distance += right_locations[index] - element,
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => distance += element - right_locations[index],
        }
    }

    println!("Question One: {}", distance);

    let mut locations_multiplied: Vec<usize> = Vec::new();

    for left_item in &left_locations {
        let mut left_count = 0;

        for right_item in &right_locations {
            if *left_item == *right_item {
                left_count += 1;
            }
        }

        if left_count > 0 {
            locations_multiplied.push(left_item * left_count);
        }
    }

    let total: usize = locations_multiplied.iter().sum();

    println!("Question two: {}", total);
}
