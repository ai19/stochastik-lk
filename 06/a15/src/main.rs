extern crate rand;

use std::env;


fn get_num_runs(args: Vec<String>)-> i32 {
    let raw_int:&String = &args[1];
    let result: i32 = raw_int.parse().unwrap();
  return result;
}

fn perform_one_random_experiment()-> f64 {
      let r1: f64 = rand::random();
      let x1 = r1*10.0;
      let r2: f64 = rand::random();
      let y1 = r2*10.0;
      let r3: f64 = rand::random();
      let x2: f64 = r3*10.0+10.0;
      let r4: f64 = rand::random();
      let y2: f64 = r4*10.0;
      let distance: f64 =  ((x2-x1)*(x2-x1) + (y2-y1)*(y2-y1)).sqrt();
    return distance;

}

fn run_experiments(num: i32) {
    let mut sum_distances = 0.0;
    println!("num runs: {:?}", num);
    for _ in 0..num {
        sum_distances += perform_one_random_experiment();
    }
    let avg_distance = (sum_distances as f64)/(num as f64);
    println!("#AVG distance: {:?}, #experiments: {:?}", avg_distance, num);
}



fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let num_runs: i32 = get_num_runs(args);
    println!("Number of runs: {:?}", num_runs);
    println!("One experiment: {:?}", perform_one_random_experiment() );
    run_experiments(num_runs);
}
