use std::env;
extern crate chrono;
use chrono::NaiveDate;

struct VacationPeriod {
    start_date: String,
    stop_date: String,
}

fn format_date(datestamp: &str) -> String {
    let parsed_date = NaiveDate::parse_from_str(datestamp, "%Y-%m-%d").unwrap();
    let formatted_date = parsed_date.format("%A, %-m/%-d").to_string();
    formatted_date
}

fn main() {
    // collect CLI arguments and skip the first argument (program name)
    let args: Vec<String> = env::args().skip(1).collect();

    // check to ensure arguments come in pairs
    if args.is_empty() || args.len() % 2 != 0 {
        println!("Invalid number of arguments. Please provide pairs of start and stop dates.");
        return;
    }
    // Simulating command line arguments for testing
    // let test_args = vec![
    //     String::from("vacation.rs"),          // Program name
    //     String::from("2023-06-01"),            // Start date 1
    //     String::from("2023-06-10"),            // Stop date 1
    //     String::from("2023-06-17"),            // Start date 2
    //     String::from("2023-06-17"),            // Stop date 2
    //     String::from("2023-07-01"),            // Start date 3
    //     String::from("2023-07-07"),            // Stop date 3
    // ];

    // let args: Vec<String> = test_args;

    // if args.len() <= 1 || args.len() % 2 != 1 {
    //     println!("Invalid number of arguments. Please provide pairs of start and stop dates.");
    //     return;
    // }

    let mut vacation_periods = Vec::new();
    for chunk in args[0..].chunks(2) {
        let start_date = chunk[0].clone();
        let stop_date = chunk[1].clone();
        let period = VacationPeriod {
            start_date: format_date(&start_date),
            stop_date: format_date(&stop_date),
        };
        vacation_periods.push(period);
    }

    println!("Subject: Upcoming vacation days\n");
    println!("Hello,\n\nI plan to take the following day(s) as vacation:");
    for period in vacation_periods {
        if period.start_date == period.stop_date {
            println!("     {}", period.start_date);
        } else {
            println!("     {} - {}", period.start_date, period.stop_date);
        }
    }
    println!("\nThank you,\nRyan");
}