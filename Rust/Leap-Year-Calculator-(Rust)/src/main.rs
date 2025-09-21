/*
Write: Nicklas Christoffersen
Date: 09-21/2025
My rewrite of "skudårsberegner" problem from school.
 */
use std::io::*;
use std::thread;
use colored::*;
use clearscreen;

fn main() {
    input_and_conversion();
}

fn input_and_conversion()
{
    clearscreen::clear().expect("Failed to clear screen");
    loop {
        println!("{}","Input year: ".bright_yellow().bold());
        let mut input_year = String::new();
        stdin().read_line(&mut input_year).expect("Failed to read line");
        match input_year.trim().parse::<i32>(){
            Ok(year) => { //unwrapping the result to int32
                calculation(year);
            }Err(_) => {println!("{}", "Invalid input".red().italic());
                thread::sleep(std::time::Duration::from_millis(500));
                clearscreen::clear().expect("Failed to clear screen"); continue;}
        }
    }
}

fn calculation(year: i32)
{
    let is_leap_year = (year % 400 == 0) || ((year % 4 == 0) && (year % 100 != 0));
    
    if is_leap_year {
        println!("{} was a leap year", year.to_string().bold().green());
        thread::sleep(std::time::Duration::from_millis(2000));
        clearscreen::clear().expect("Failed to clear screen");
    } else {
        println!("{} was not a leap year", year.to_string().bright_magenta().bold());
        thread::sleep(std::time::Duration::from_millis(2000));
        clearscreen::clear().expect("Failed to clear screen");
    }
}