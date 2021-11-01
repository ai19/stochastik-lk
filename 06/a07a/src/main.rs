extern crate rand;

use std::env;

static PROBABILITY: f64 = 1.0/6.0;

fn get_num_runs(args: Vec<String>)-> i32 {
    let raw_int:&String = &args[1];
    let result: i32 = raw_int.parse().unwrap();
  return result;
}

fn perform_one_random_experiment()-> Vec<i32> {
    let mut result: Vec<i32> = vec![0, 0, 0, 0];
    for i in 0..4 {
      let r: f64 = rand::random();
      if r < PROBABILITY {
          result[i] = 1;
      } 
      if 1.0*PROBABILITY <= r && r < 2.0*PROBABILITY { result[i] = 2; }
      if 2.0*PROBABILITY <= r && r < 3.0*PROBABILITY { result[i] = 3; }
      if 3.0*PROBABILITY <= r && r < 4.0*PROBABILITY { result[i] = 4; }
      if 4.0*PROBABILITY <= r && r < 5.0*PROBABILITY { result[i] = 5; }
      if 5.0*PROBABILITY <= r  { result[i] = 6; }
      
//      println!("i: {:?}", i);
    }
    return result;
}

fn is_case_of_interest(experiment: Vec<i32>) -> bool {
  let mut num_groesser_vier = 0;

  for augenzahl in experiment {
      if augenzahl > 4 {
          num_groesser_vier += 1;
      }
  }

  if num_groesser_vier > 2 {
      return true;
  } else {
      return false;
  }

}

fn run_experiments(num: i32) {
    let mut num_cases_of_interest = 0;
    println!("num runs: {:?}", num);
    for _ in 0..num {
        let result = perform_one_random_experiment();
        if is_case_of_interest(result) { num_cases_of_interest +=1 };
    }
    let probability = (num_cases_of_interest as f64)/(num as f64);
    println!("#Cases of interest: {:?}, #experiments: {:?}, #probability: {:?}", num_cases_of_interest, num, probability);
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
