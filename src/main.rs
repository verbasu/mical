extern crate ical;

use ical::generator::{Emitter, IcalCalendarBuilder};
use ical::IcalParser;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let mut cal = IcalCalendarBuilder::version("2.0")
        .gregorian()
        .prodid("-//ical-rs//github.com//")
        .build();

    let buf = BufReader::new(File::open("calcurse.ical").unwrap());
    let reader = IcalParser::new(buf);
    //let reader = ical::PropertyParser::from_reader(buf);

    match reader.last() {
        Some(Ok(i)) => {
            //for line in i.events {
            //    println!("{:?}", line);
            //}
            cal.events.push(i.events.last().unwrap().clone());
            print!("{}", cal.generate());
        }
        Some(Err(e)) => println!("{:?}", e),
        None => println!("None"),
    }
}
