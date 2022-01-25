extern crate rand;

use std::env;

//static KOPF: i32 = 1;
//static ZAHL: i32 = 2;


fn get_num_runs(args: Vec<String>)-> i32 {
    let raw_int:&String = &args[1];
    let result: i32 = raw_int.parse().unwrap();
  return result;
}

fn perform_one_random_experiment()-> i32 {
    let mut result: i32 = 0;
    let mut num_kopf: i32 = 0;

    loop {
      let r: f64 = rand::random();
      if r<0.5 {
          result += 1;
          num_kopf += 1;
      } else {
          result += 1;
      }

      if num_kopf == 3 {
          return result;
      }
    }
}


fn run_experiments(num: i32) {
    let mut num_throws = 0;
    println!("num runs: {:?}", num);
    for _ in 0..num {
        let result = perform_one_random_experiment();
//        println!("{:?}",result);
        num_throws += result;
    }
    let num_avg_throws = num_throws / num;
    println!("#experiments: {:?}, #:vg throws {:?}", num, num_avg_throws);
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
