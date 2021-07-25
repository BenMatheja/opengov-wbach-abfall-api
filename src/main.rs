
extern crate ical;

use std::io::BufReader;
use std::fs::File;
use ical::parser::ical::component::IcalEvent;
use curl::easy::Easy;

// we want to retrieve the ics file from 
// https://calendar.google.com/calendar/ical/9k9n7fm5qgjp6qkre2hr2p6irc%40group.calendar.google.com/public/basic.ics

fn main() {
    let mut handle = Easy::new();
    let mut buf = Vec::new();
    handle
        .url("https://calendar.google.com/calendar/ical/9k9n7fm5qgjp6qkre2hr2p6irc%40group.calendar.google.com/public/basic.ics")
        .unwrap();

    let mut transfer = handle.transfer();
    transfer.write_function(|data| {
            buf.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    
    
    let filebuf = BufReader::new(File::open("basic.ics").unwrap());
    let reader = ical::IcalParser::new(filebuf);
    let mut events : Vec<IcalEvent> = Vec::new();
    // Read all lines from the buffered reader
    let mut i = 0;
    for line in reader {
        // This is a Vec<IcalEvent>
        // TODO There must be a smarter way to do this than two for loops
        for element in line  {
            //println!("{:?}", element.events);
            for single_event in element.events {        
                // i += 1;
                // println!("{}", i);
                // //println!("{:?}",single_event);
                // println!("{:?}",single_event.properties.get(10));
                // println!(" \n ");
                events.push(single_event);
                
            }
        }
    }

    //println!("{:?}", events);
          
}

//element.events[0].properties[10].name


