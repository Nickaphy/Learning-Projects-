use std::io::*;
use colored::*;
use clearscreen;
use std::thread::sleep;
use std::time::Duration;

fn main()
{
    clearscreen::clear().expect("Couldnt clear screen");
    calc();
}

fn calc()
{
    let mut i = 0;
    let mut max = 0;
    let attempts = 2;

    for _ in 0..100 //a compromise for using for loop. So misinputs wont take an attempt
    {
        if i > attempts {break;}

        println!("{}", "A whole number: ".blue().bold());
        let mut num= String::new();
        stdin().read_line(&mut num).expect("TODO: panic message");

        match num.trim().parse::<i32>(){
            Ok(number) => {
                i += 1; //only increment if input was valid
                if number > max {
                    max = number;
                    clearscreen::clear().expect("Couldnt clear screen");
                }
            }Err(_) => {println!("{}","Not a whole number".red().italic());
                sleep(Duration::from_millis(1000)); clearscreen::clear().expect("Couldnt clear screen");
                continue;
            }
        }
    }
    println!("The biggest number is {}", max.to_string().green().bold());
}