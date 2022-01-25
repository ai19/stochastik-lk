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
    println!("{:?}", result);
    return result;
}

fn is_case_of_interest(experiment: Vec<i32>) -> bool {
  // wir checken als positiv:
  // - eine Zahl wurde 4 mal gewürfelt
  // - eine Zahl wurde 3 mal gewürfelt
  // - zwei Zahlen wurden zweimal gewürfelt
  //
  let mut wie_oft_zweimal = 0;
  let mut result = false;

  let mut haeufigkeit_augenzahlen: Vec<i32> = vec![0, 0, 0, 0, 0, 0];

  for augenzahl in experiment {
      let index = augenzahl as usize - 1;
      haeufigkeit_augenzahlen[index] += 1;
  }

  for haeufigkeit in haeufigkeit_augenzahlen {
      if haeufigkeit == 4 { result = true; }
      if haeufigkeit == 3 { result = true; }
      if haeufigkeit == 2 { wie_oft_zweimal += 1}
  }
  if wie_oft_zweimal == 2 { result = true; }

  return result;
}

fn run_experiments(num: i32) {
    let mut num_cases_of_interest = 0;
    println!("num runs: {:?}", num);
    for _ in 0..num {
        let result = perform_one_random_experiment();
        if is_case_of_interest(result) { 
            num_cases_of_interest +=1;
            println!("Check.");
        };
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
