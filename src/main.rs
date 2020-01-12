use std::fs;

fn main() {
    let input = fs::read_to_string("input/1.txt").expect("Error reading the file");
    let masses = input.lines().map(|line: &str| line.parse::<u128>().expect("couldn't parse int"));
    let mut total: u128 = 0;
    for mass in masses {
        total += calculate_incremental_required_fuel(mass);
    }
    println!("{}", total);
}

fn calculate_required_fuel(mass: u128) -> u128 {
  return (mass / 3).checked_sub(2).unwrap_or_else(|| 0);
}

fn calculate_incremental_required_fuel(mass: u128) -> u128 {
    let mut total_mass: u128 = 0;
    let mut fuel_needed: u128 = calculate_required_fuel(mass);
    loop {
        if fuel_needed <= 0 {
            return total_mass;
        }
        total_mass += fuel_needed;
        fuel_needed = calculate_required_fuel(fuel_needed);
    }
}
