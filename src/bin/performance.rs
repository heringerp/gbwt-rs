use gbwt::GBZ;

use simple_sds::serialize;

use std::time::Instant;
use std::{env, process};

use getopts::Options;
use rayon::prelude::*;

//-----------------------------------------------------------------------------

fn main() {
    let config = Config::new();

    let filename = config.filename.as_ref().unwrap();
    println!("Loading GBZ index {}", filename);
    let gbz_graph: GBZ = serialize::load_from(filename).unwrap();

    let now = Instant::now();
    let steps: Vec<_> = (0..gbz_graph.paths())
        .into_par_iter()
        .map(|id| {
            (
                gbz_graph.metadata().map(|m| m.sample(id)),
                gbz_graph
                    .segment_path(id, gbwt::Orientation::Forward)
                    .expect("No sequence for id ")
                    .count(),
            )
        })
        .collect();

    // for (_, step_count) in steps {
    // for (entry, _) in step_count {
    //     print!("{}+,", entry);
    // }
    // println!("");
    // println!("{:?}", step_count)
    // match name {
    //     Some(n) => match n {
    //         Some(n_t) => println!("no. steps for path {}: {:?}", n_t, step_count),
    //         None => println!("no. steps for path UNKNOWN: {:?}", step_count),
    //     },
    //     None => println!("no. steps for path unknown: {:?}", step_count),
    // }
    // }

    eprintln!("Time: {}ms", now.elapsed().as_millis());
    //internal::report_memory_usage();
}

//-----------------------------------------------------------------------------

pub struct Config {
    pub filename: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        let args: Vec<String> = env::args().collect();
        let program = args[0].clone();

        let mut opts = Options::new();
        opts.optflag("h", "help", "print this help");
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => {
                eprintln!("{}", f.to_string());
                process::exit(1);
            }
        };

        let mut config = Config { filename: None };
        if matches.opt_present("h") {
            let header = format!("Usage: {} [options] graph.gbz", program);
            eprint!("{}", opts.usage(&header));
            process::exit(0);
        }

        if !matches.free.is_empty() {
            config.filename = Some(matches.free[0].clone());
        } else {
            let header = format!("Usage: {} [options] graph.gbz", program);
            eprint!("{}", opts.usage(&header));
            process::exit(1);
        }

        config
    }
}
