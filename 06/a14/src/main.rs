extern crate rand;

use std::env;


fn get_num_runs(args: Vec<String>)-> i32 {
    let raw_int:&String = &args[1];
    let result: i32 = raw_int.parse().unwrap();
  return result;
}

fn perform_one_random_experiment()-> (i32, i32) {
  let mut result: i32 =0;
  let mut num_hops_on_zero: i32 = 0;
  for _ in 0..10 {
    let r: f64 = rand::random();
      if r<0.5 {
        result += 1;
      } else {
        result -= 1;
      }
      if result == 0 { num_hops_on_zero += 1; }
  }
  return (result, num_hops_on_zero);
}

fn is_case_of_interest_a(experiment: i32) -> bool {
  if experiment >= 4 {
      return true;
  } else {
      return false;
  }
}


fn is_case_of_interest_b(experiment: i32) -> bool {
  if experiment == 0 {
      return true;
  } else {
      return false;
  }
}


fn run_experiments(num: i32) {
    let mut num_cases_of_interest_a = 0;
    let mut num_cases_of_interest_b = 0;
    let mut sum_results = 0;
    let mut sum_num_hops_in_zero = 0;
    println!("num runs: {:?}", num);
    for _ in 0..num {
        let (result, num_hops_in_zero) = perform_one_random_experiment();
        if is_case_of_interest_a(result) { num_cases_of_interest_a +=1 };
        if is_case_of_interest_b(result) { num_cases_of_interest_b +=1 };
        sum_results += result;
        sum_num_hops_in_zero += num_hops_in_zero;
    }
    let probability_a = (num_cases_of_interest_a as f64)/(num as f64);
    let probability_b = (num_cases_of_interest_b as f64)/(num as f64);
    let mean_hops_zero = (sum_num_hops_in_zero as f64)/(num as f64);
    let mean_position = (sum_results as f64)/(num as f64);
    println!("#Cases of interest a: {:?}, #experiments: {:?}, #probability: {:?}", num_cases_of_interest_a, num, probability_a);
    println!("#Cases of interest b: {:?}, #experiments: {:?}, #probability: {:?}", num_cases_of_interest_b, num, probability_b);
    println!("AVG times in 0: {:?}, #experiments: {:?}", mean_hops_zero, num);
    println!("AVG position after 10 hops: {:?}, #experiments: {:?}", mean_position, num);
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
