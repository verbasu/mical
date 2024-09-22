use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use ical::generator::{Emitter, IcalCalendarBuilder, IcalEventBuilder, Property};
use ical::ical_property;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use uuid::Uuid;

struct Appointment {
    date_time_start: DateTime<Local>,
    date_time_end: DateTime<Local>,
    description: String,
}

fn parse_date_time(date_str: &str) -> DateTime<Local> {
    let from = NaiveDateTime::parse_from_str(date_str, "%m/%d/%Y @ %H:%M").unwrap();
    return Local.from_local_datetime(&from).unwrap();
}

fn parse_appointments(file_path: &str) -> Result<Vec<Appointment>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    let re = Regex::new(
        r"(\d{2}\/\d{2}\/\d{4} @ \d{2}:\d{2}) -> (\d{2}\/\d{2}\/\d{4} @ \d{2}:\d{2})(.*)[\||\!](.*)",
    )?;

    let mut appointments: Vec<Appointment> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if re.is_match(&line) {
                    let caps = re.captures(&line).unwrap();

                    let date_time_start = caps.get(1).map_or("", |m| m.as_str());
                    let date_time_end = caps.get(2).map_or("", |m| m.as_str());
                    let description = caps.get(4).map_or("", |m| m.as_str());

                    appointments.push(Appointment {
                        date_time_start: parse_date_time(&date_time_start),
                        date_time_end: parse_date_time(&date_time_end),
                        description: description.to_string(),
                    });
                }
            }
            Err(e) => return Err(Box::new(e)),
        }
    }

    Ok(appointments)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "/home/ikichigai/.local/share/calcurse/apts";

    let appointments = parse_appointments(file_path)?;

    let mut cal = IcalCalendarBuilder::version("2.0")
        .gregorian()
        .prodid("-//ical-rs//github.com//")
        .build();

    println!("Parsed Appointments:");

    for appointment in appointments.iter() {
        let event = IcalEventBuilder::tzid("Europe/London")
            .uid(Uuid::new_v4())
            .changed(Local::now().format("%Y%m%dT%H%M%S").to_string())
            .start(
                appointment
                    .date_time_start
                    .format("%Y%m%dT%H%M%S")
                    .to_string(),
            )
            .end(
                appointment
                    .date_time_end
                    .format("%Y%m%dT%H%M%S")
                    .to_string(),
            )
            .set(ical_property!("SUMMARY", appointment.description.clone()))
            .build();
        cal.events.push(event);
    }
    print!("{}", cal.generate());
    Ok(())
}
