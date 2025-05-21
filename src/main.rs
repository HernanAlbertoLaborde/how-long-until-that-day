/*
This project helps you know how many days, hours, and minutes 
are left until a target timestamp.
The input from console are:
- Event Tag: Title to identify an event.
- Target Date: Year, month and day of the event in format YYYY-MM-DD
- Target Time: Hour, minutes and seconds of the target event in format HH-MM-SS.

Usage example:
cargo run -- -t "Salida Petro" -d "2026-08-07" 
*/
use clap::{command, Arg, ArgGroup};
use chrono::{NaiveDate, Local};

fn main() {
    println!("How long until that day...");

    let matches = command!()
        .about("This program helps you know how many days, hours, and minutes are left until a target timestamp.")

        .group(
            ArgGroup::new("event")
                .arg("event_title")
                .arg("target_date")
                .multiple(true)  // if false is one or another but not both.
        )
        //.group(ArgGroup::new("helth check"))
        .arg(
            Arg::new("event_title") 
                .short('t')
                .long("title")
                .aliases(["EventTitle", "Event-Title"])
                .help("The Title of the event")
                //.conflicts_with("lastname")
                .required(true),
        )
        .arg(
            Arg::new("target_date")
                .short('d')
                .long("target-date")
                .aliases(["TargetDate", "Target-Date"])
                .help("The target date of the event in format YYYY-MM-DD")
                .required(true),
            )
        .get_matches();

    // Get the value from the matches
    let event_title = matches.get_one::<String>("event_title").unwrap();
    let target_date = matches.get_one::<String>("target_date").unwrap();

    // Parse the target date
    let target = NaiveDate::parse_from_str(target_date, "%Y-%m-%d")
        .expect("Invalid date format, use YYYY-MM-DD");

    // Get today's date
    let today = Local::now().date_naive();

    // Compute the difference
    let days_left = (target - today).num_days();

    //print first and lastname
    println!("Title: {} Target Date: {} Days left: {}", event_title, target_date, days_left);

}

