use clap::{arg, command, value_parser, ArgAction};
use rand::distributions::{Alphanumeric, DistString};
use std::env;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader, Write};
use std::process::ExitCode;
use std::process::{Command, Stdio};
use std::time::Instant;

fn main() -> ExitCode {
    let matches = command!()
        .about("Measure SSH roundtrip latency")
        .arg(
            arg!(--iterations <n>)
                .value_parser(value_parser!(i64))
                .default_value("20")
                .required(false)
                .help("Number of measurements to perform"),
        )
        .arg(arg!([command] "Command to run (typically ssh)").required(true))
        .arg(
            arg!([args] "Arguments to supply to command (typically a hostname)")
                .action(ArgAction::Append),
        )
        .after_help("Roundtrip times are printed in whole microseconds.\n\
                     The measurement works by executing commands of the form 'echo \u{1b}[3m<message>\u{1b}[23m' where \u{1b}[3m<message>\u{1b}[23m \
                     is a randomly chosen alphanumeric character. Roundtrip time is measured as the time between \
                     the command being supplied on stdin, and the response (which should be identical to \u{1b}[3m<message>\u{1b}[23m) \
                     being read back from stdout. \
                     Before measurements start, an initial 'echo \u{1b}[3m<message>\u{1b}[23m' is executed with a randomly-chosen \
                     32-character alphanumeric message. As soon as this message is seen on stdout, we assume that \
                     any SSH welcome messages have been processed and the remote host is ready for measurement.
                     ")
        .get_matches();

    let iterations = *matches.get_one::<i64>("iterations").unwrap();
    let command = matches.get_one::<String>("command").unwrap();
    let options: Vec<_> = matches
        .get_many::<String>("args")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect();
    let mut child = Command::new(OsStr::new(&command))
        .args(options)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to run command");
    let k = child.stdin.as_mut().unwrap();
    let mut g = BufReader::new(child.stdout.as_mut().unwrap());

    let mut lines_skipped = 0;
    let first_message = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
    k.write_all(format!("echo {}\n", first_message).as_bytes())
        .unwrap();
    loop {
        let mut response: String = String::new();
        g.read_line(&mut response).unwrap();
        let response = response.trim();
        if response == first_message {
            break;
        }
        lines_skipped += 1;
        if lines_skipped > 1000 {
            println!("More than 1000 lines read before match. Exiting.");
            return ExitCode::from(2);
        }
    }

    for _ in 0..iterations {
        let message = Alphanumeric.sample_string(&mut rand::thread_rng(), 1);
        let mut response: String = String::new();

        k.write_all(format!("echo {}\n", message).as_bytes())
            .unwrap();
        let now = Instant::now();
        g.read_line(&mut response).unwrap();
        let dt = now.elapsed();

        let response = response.trim();
        if response != message {
            panic!("did not match: '{}' and '{}'", message, response);
        }
        println!("{}\r", dt.as_micros());
    }
    ExitCode::from(0)
}
