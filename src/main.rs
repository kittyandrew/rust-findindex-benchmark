use std::time::SystemTime;
use std::process::exit;
use std::fs::File;
use std::io::Read;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("\nUSAGE: rust-shakespeare <haystackfile> <needle>\n\n");
        println!(" <haystackfile> = path of file containing haystack text\n");
        println!(" <needle>       = text to search for\n");
        exit(1);
    }

    let haystack_file = &args[1];
    let needle        = &args[2];


    // Reading input.
    println!("Reading '{}' ...", &haystack_file);

    let contents = match File::open(&haystack_file) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Couldn't read file.");
            contents
        },
        Err(_) => {
            println!("Uh-oh failed to open the file '{}'!", &haystack_file);
            exit(1);
        }
    };

    let mut start: SystemTime;
    let total_time_left:  f64;
    let total_time_right: f64;

    let times = 100;    
    let mut total_time_micros_left:  u128 = 0;
    let mut total_time_micros_right: u128 = 0;

    let mut total_work:   u64 = 0;

    // From the left.

    println!("Doing {} iterations of search for all instances of '{}' from the left ...", times, needle);
    for _ in 1..=times {  // With the help of ..= we made range inclusive on both ends.
        start = SystemTime::now();

        for m in contents.match_indices(needle) {
            total_work += m.0 as u64;
        }

        //println!("Output all found indecies: {:?}", indecies);
        //println!("Found indecies: {:?}", indecies.len());

        total_time_micros_left += SystemTime::now().duration_since(start).unwrap().as_micros();
    }

    // From the right.

    println!("Doing {} iterations of search for all instances of '{}' from the right ...", times, needle);
    for _ in 1..=times {  // With the help of ..= we made range inclusive on both ends.
        start = SystemTime::now();

        for m in contents.rmatch_indices(needle) {
            total_work += m.0 as u64;
        }

        //println!("Output all found indecies: {:?}", indecies);
        //println!("Found indecies: {:?}", indecies.len());

        total_time_micros_right += SystemTime::now().duration_since(start).unwrap().as_micros();
    }

    total_time_left = total_time_micros_left as f64 / 1_000_000.;
    total_time_right = total_time_micros_right as f64 / 1_000_000.;

    println!("Final total time: {} / {} seconds!", total_time_left, total_time_right);
    println!("Per iter (N={}): {:.10} / {:.10} seconds.", times as f64, total_time_left / times as f64, total_time_right / times as f64);
    println!("Result (total_work): {}", total_work);
}

