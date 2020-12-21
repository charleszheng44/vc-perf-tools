use serde::{Deserialize, Serialize};
use serde_json;
mod rand_num_gen;
use std::env;
use std::fs::{self, OpenOptions};
use std::path::Path;
use std::io::prelude::*;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Tenant {
    id: String,
    numPods: u32, 
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        panic!("invalid number of arguments, a correct command looks like: \
            ./tenant_gen <num_tenants>, <min>, <max>, <total_num_pods>");
    }
    
    let (num_tenants, min_pods, max_pods, total_num_pods) = 
        (args[1].parse::<u32>().
         expect("fail to parse the command line argument <num_tenants>"), 
         args[2].parse::<u32>().
         expect("fail to parse the command line argument <min_pods>"), 
         args[3].parse::<u32>().
         expect("fail to parse the command line argument <max_pods>"), 
         args[4].parse::<u32>().
         expect("fail to parse the command line argument <total_num_pods>"));

    
    let rand_nums = rand_num_gen::gen_rand_nums(num_tenants, min_pods, max_pods, total_num_pods);
    
    let mut ts: Vec<Tenant> = vec![];
    for i in 0..num_tenants {
        let t = Tenant {
            id: format!("t{}", i+1),
            numPods: rand_nums[i as usize],
        };
        ts.push(t);
    }

    let json_str = serde_json::to_string_pretty(&ts).
        expect("fail to serialize tenant");

    // create the json file
    let file_name = format!("{}-tenants-random.json", num_tenants);

    if Path::new(&file_name).exists() {
        fs::remove_file(&file_name).
            expect(&format!("fail to remove file {}", file_name));
    }

    let mut json_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(file_name)
        .expect("fail to open the json file");
    
    writeln!(json_file, "{}", json_str).
        expect("fail to write to the json file");
    
}
