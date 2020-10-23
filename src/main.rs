use std::fs::File;
use std::io::SeekFrom;
use std::io::BufReader;
use std::io::prelude::*;
use std::{thread};
use std::time::{SystemTime, Duration};
use subprocess::{Popen, PopenConfig, Redirection};
use std::error::Error;
use std::env;

fn start_process(command: &std::string::String) -> subprocess::Popen {
    let p = Popen::create(&["/bin/bash", "-c", command], PopenConfig {
        stdout: Redirection::File(File::open("/dev/null").unwrap()),
        stderr: Redirection::File(File::open("/dev/null").unwrap()),
        ..Default::default()
    }).unwrap();

    return p;
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file = File::open("/proc/cpuinfo")?;
    let delay = Duration::from_millis(1);
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    let mut top: i16;

    let start = SystemTime::now();
    let mut loop_counter: u128 = 1;

    let command = &args[1];
    let mut p = start_process(command);

    top = 0;

    loop {
        buf_reader.read_to_string(&mut contents)?;
        let v: Vec<&str> = contents.split('\n').collect();

        for line in v {
            if line.starts_with("cpu MHz") {
                let mhz_str: &str = &line[11..15];
                let mhz: i16 = mhz_str.parse().unwrap();
                if mhz > top {
                    top = mhz;
                }
            }
        }

        thread::sleep(delay);

        let duration = SystemTime::now().duration_since(start).expect("Time went backwards");
        let millis = duration.as_millis();

        if millis > (250 * loop_counter) {
            loop_counter += 1;

            println!("{}", top);
            buf_reader.seek(SeekFrom::Start(0))?;

            contents.clear();
            top = 0;

            if millis > 10 * 1000 {
                p.terminate()?;
                break;
            }
        }
    }

    Ok(())
}
