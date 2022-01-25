extern crate rand;

use std::env;


fn get_num_runs(args: Vec<String>)-> i32 {
    let raw_int:&String = &args[1];
    let result: i32 = raw_int.parse().unwrap();
  return result;
}

fn perform_one_random_experiment()-> Vec<i32> {
    let mut result: Vec<i32> = vec![0, 0, 0, 0, 0, 0];
    let mut i = 0;
    while i < 6 {
      let r: f64 = rand::random();
      let tmp: i32 = (49.0*r).ceil() as i32;
      let mut equal = false;
      for j in 0..6 {
        if result[j] == tmp { equal = true; }
      }
      if !equal { 
          result[i] = tmp;
          i += 1;
      }
    
    }
//      println!("i: {:?}", i);
    result.sort();
//    println!("{:?}", result);
    return result;
}

fn is_case_of_interest(experiment: Vec<i32>) -> bool {
  // hier betrachten wir das Gegenereignis und geben die Negation zurück.
  //
  let mut aufeinanderfolgend = false;

  for i in 0..5 {
      if experiment[i+1]-experiment[i] == 1 { aufeinanderfolgend = true; }
  }
  return aufeinanderfolgend;
}

fn run_experiments(num: i32) {
    let mut num_cases_of_interest = 0;
    println!("num runs: {:?}", num);
    for _ in 0..num {
        let result = perform_one_random_experiment();
        if is_case_of_interest(result) { num_cases_of_interest +=1 };
//        println!("{:?}",result);
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
