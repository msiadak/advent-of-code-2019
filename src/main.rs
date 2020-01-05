use std::fs;

fn main() {
    let input = fs::read_to_string("input/1.txt").expect("Error reading the file");
    let masses = input.lines().map(|line: &str| line.parse::<u128>().expect("couldn't parse int"));
    let mut total: u128 = 0;
    for mass in masses {
        total += incremental_rocket_equation(mass);
    }
    println!("{}", total);
}

fn rocket_equation(mass: u128) -> u128 {
  return (mass / 3).checked_sub(2).unwrap_or_else(|| 0);
}

fn incremental_rocket_equation(mass: u128) -> u128 {
    let mut total_mass: u128 = 0;
    let mut fuel_needed: u128 = rocket_equation(mass);
    loop {
        if fuel_needed <= 0 {
            return total_mass;
        }
        total_mass += fuel_needed;
        fuel_needed = rocket_equation(fuel_needed);
    }
}
