extern crate ical;

use ical::generator::{Emitter, IcalCalendarBuilder, IcalEventBuilder, Property};
use ical::ical_property;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Appointment {
    date_time_start: String,
    date_time_end: String,
    description: String,
}

fn parse_appointments(file_path: &str) -> Result<Vec<Appointment>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    let re = Regex::new(
        r"(\d{2}\/\d{2}\/\d{4} @ \d{2}:\d{2}) -> (\d{2}\/\d{2}\/\d{4} @ \d{2}:\d{2})(.*) [\||\!](.*)",
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
                        date_time_start: date_time_start.to_string(),
                        date_time_end: date_time_end.to_string(),
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

    let cal = IcalCalendarBuilder::version("2.0")
        .gregorian()
        .prodid("-//ical-rs//github.com//")
        .build();

    println!("Parsed Appointments:");

    for appointment in appointments.iter() {
        println!(
            "- Date Start: {}, Date End: {}, Description: {}",
            appointment.date_time_start, appointment.date_time_end, appointment.description
        );
        //let event = IcalEventBuilder::tzid("Europe/London")
        //    .uid("UID for identifying this event.")
        //    .set(ical_property!("SUMMARY", appointment.description))
        //    .build();
        //cal.events.push(event);
    }
    cal.generate();
    Ok(())
}
