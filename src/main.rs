use colored::*;

fn main() {
    let deg_from: i32 = get_type(0);
    let deg_to: i32 = get_type(1);
    let temp: f32 = get_temp();
    let mut val_from = "";
    let mut val_to = "";
    match deg_from {
        1 => val_from = "C",
        2 => val_from = "F",
        3 => val_from = "K",
        _ => ()
    }
    match deg_to {
        1 => val_to = "C",
        2 => val_to = "F",
        3 => val_to = "K",
        _ => ()
    }
    println!("{}: {}°{} to {}°{}", "Final conversion".green(), temp.to_string(), val_from, convert((deg_from, deg_to, temp)).to_string(), val_to);
}

// Get temperature value
fn get_temp() -> f32 {
    loop {
        let mut line = String::new();
        println!("{} Enter the temperature: ", "?".blue());
        std::io::stdin().read_line(&mut line).expect("Failed to read line");
        match line.trim().parse::<f32>() {
            Ok(temp) => {
                println!("{{ Temperature: {}{} }}", temp.to_string().blue().bold(), "°C/F/K".blue().bold());
                return temp;
            }
            Err(_) => { eprintln!("{}", "Invalid number — please try again.".red()); }
        }
    }
}

// Celsius, Fahrenheit, Kelvin
fn get_type(x: i32) -> i32 {
    loop {
        let mut line = String::new();
        let mut y = "input type";
        if x == 1 { y = "type to convert to" }
        println!("{} Celsius\n{} Fahrenheit\n{} Kelvin\n{} Enter the {}: ", "[1]".bold(), "[2]".bold(), "[3]".bold(), "?".blue(), y);
        std::io::stdin().read_line(&mut line).expect("Failed to read line");
        match line.trim().parse::<i32>() {
            Ok(deg) => {
                if deg == 1 {
                    let x = "Celsius";
                    println!("{{ Type: {} }}", x.to_string().blue().bold());
                    return deg;
                } else if deg == 2 {
                    let x = "Fahrenheit";
                    println!("{{ Type: {} }}", x.to_string().blue().bold());
                    return deg;
                } else if deg == 3 {
                    let x = "Kelvin";
                    println!("{{ Type: {} }}", x.to_string().blue().bold());
                    return deg;
                } else {
                    eprintln!("{}", "Invalid choice — please try again.".red()); 
                }
            }
            Err(_) => { eprintln!("{}", "Invalid choice — please try again.".red()); }
        }
    }
}

// Conversions
fn convert((deg_from, deg_to, temp): (i32, i32, f32)) -> f32 { 
    // °F = (°C × 9/5) + 32
    if deg_from == 1 && deg_to == 2 { return (temp * 9.0/5.0) + 32.0; }
    // °K = °C + 273.15
    if deg_from == 1 && deg_to == 3 { return temp + 273.15; }
    // °C = (°F - 32) × 5/9
    if deg_from == 2 && deg_to == 1 { return (temp - 32.0) * 5.0/9.0; }
    // °K = (°F - 32) × 5/9 + 273.15
    if deg_from == 2 && deg_to == 3 { return (temp - 32.0) * 5.0/9.0 + 273.15; }
    // °C = °K - 273.15
    if deg_from == 3 && deg_to == 1 { return temp - 273.15; }
    // °F = (°K - 273.15) × 9/5 + 32
    if deg_from == 3 && deg_to == 2 { return (temp - 273.15) * 9.0/5.0 + 32.0; }
    temp
}