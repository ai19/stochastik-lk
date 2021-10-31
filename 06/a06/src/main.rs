extern crate rand;

use std::env;


static JUNGE: i32 = 1;
static MAEDCHEN: i32 = 2;


fn get_num_runs(args: Vec<String>)-> i32 {
    let raw_int:&String = &args[1];
    let result: i32 = raw_int.parse().unwrap();
  return result;
}

fn perform_one_random_experiment()-> Vec<i32> {
    let mut result: Vec<i32> = vec![0, 0, 0, 0, 0];
    for i in 0..5 {
      let r: f64 = rand::random();
      if r < 0.514 {
          result[i] = JUNGE;
      } else {
          result[i] = MAEDCHEN;
      }
//      println!("i: {:?}", i);
    }
    return result;
}

fn is_case_of_interest(experiment: Vec<i32>) -> bool {
  let mut num_maedchen = 0;

  for gender in experiment {
      if gender == MAEDCHEN {
          num_maedchen += 1;
      }
  }

  if num_maedchen >= 3 {
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
