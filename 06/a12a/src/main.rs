extern crate rand;

use std::env;

static WHITE: i32 = 1;
static BLACK: i32 = 2;


fn get_num_runs(args: Vec<String>)-> i32 {
    let raw_int:&String = &args[1];
    let result: i32 = raw_int.parse().unwrap();
  return result;
}

fn perform_one_random_experiment()-> Vec<i32> {
    let mut num_white: i32 = 3;
    let mut num_black: i32 = 7;
    let mut result: Vec<i32> = vec![0,0,0,0,0,0,0,0,0,0];

    for i in 0..10 {
      let r: f64 = rand::random();
      if r < num_white as f64 / (num_white+num_black) as f64 {
          result[i] = WHITE;
          num_white -= 1;
      } else {
          result[i] = BLACK;
          num_black -= 1;
      }
    }
    return result;

}

fn is_case_of_interest(experiment: Vec<i32>) -> bool {
  let mut first_white = 0;

  for i in 0..10 {
      if experiment[i] == WHITE {
          first_white = i;
          break;
      }
  }
  if first_white == 1 { 
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
