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
    let needle = &args[2];


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

    // Looking for needle.
    let times = 100;    
    let mut total_time_micros: u128 = 0;

    println!("Doing {} iterations of search for all instances of '{}' ...", times, needle);

    for _ in 1..=times {  // With the help of ..= we made range inclusive on both ends.
        let start = SystemTime::now();

        let mut indecies = Vec::new();
        for m in contents.match_indices(needle) {
            indecies.push(m.0);
        }

        //println!("Output all found indecies: {:?}", indecies);
        //println!("Found indecies: {:?}", indecies.len());

        total_time_micros += SystemTime::now().duration_since(start).unwrap().as_micros();
    }
    let total_time = total_time_micros as f64 / 1_000_000.;
    println!("Final total time: {} seconds!", total_time);
    println!("Per iter (N={}): {}", times as f64, total_time / times as f64);
}

