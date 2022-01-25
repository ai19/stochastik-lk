extern crate rand;

use std::env;


fn get_num_runs(args: Vec<String>)-> i32 {
    let raw_int:&String = &args[1];
    let result: i32 = raw_int.parse().unwrap();
  return result;
}

fn perform_one_random_experiment()-> i32 {
//    let mut result: i32;
    let r: f64 = rand::random();
    let result = (37.0*r).floor() as i32;
    return result;
}

fn is_douze_milieu(n: i32) -> bool {
    if n>= 13 && n<=24 {
        return true;
    } else {
        return false;
    }
}

fn is_manque(n: i32) -> bool {
    if n>=1 && n <= 18 {
        return true;
    } else {
        return false;
    }
}

fn is_transversale_pleine(n: i32) -> bool {
    if n >= 10 && n <= 12 {
        return true;
    } else {
        return false;
    }
}

fn is_transversale_simple(n: i32) -> bool {
    if n >= 7 && n <= 12 {
        return true;
    } else {
        return false;
    }
}

fn is_carre(n: i32) -> bool {
    if n==8 || n == 9 || n==11 || n==12 {
        return true; 
    } else {
        return false;
    }
}


fn run_experiments(num: i32) {
    let mut num_douze_milieu = 0;
    let mut num_manque = 0;
    let mut num_transversale_pleine = 0;
    let mut num_transversale_simple = 0;
    let mut num_carre = 0;
    let mut num_zeros = 0;
    println!("num runs: {:?}", num);
    for _ in 0..num {
        let result = perform_one_random_experiment();
        if is_douze_milieu(result) { num_douze_milieu +=1 };
        if is_manque(result) { num_manque +=1; }
        if is_transversale_pleine(result) { num_transversale_pleine += 1; }
        if is_transversale_simple(result) { num_transversale_simple += 1; }
        if is_carre(result) { num_carre += 1; }
        if result == 0 { num_zeros += 1; }
//        println!("{:?}",result);
    }
    let p_douze_milieu = (num_douze_milieu as f64)/(num as f64);
    let p_manque = (num_manque as f64)/(num as f64);
    let p_transversale_plein = (num_transversale_pleine as f64) / (num as f64);
    let p_transversale_simple = (num_transversale_simple as f64) / (num as f64);
    let p_carre = (num_carre as f64) / (num as f64);
    let p_zeros = (num_zeros as f64) / (num as f64);
    println!("#Douze Milieu: {:?}, #experiments: {:?}, #probability: {:?}", num_douze_milieu, num, p_douze_milieu);
    println!("#Manque: {:?}, #experiments: {:?}, #probability: {:?}", num_manque, num, p_manque);
    println!("#Transversale Pleine: {:?}, #experiments: {:?}, #probability: {:?}", num_transversale_pleine, num, p_transversale_plein);
    println!("#Transversale Simple: {:?}, #experiments: {:?}, #probability: {:?}", num_transversale_simple, num, p_transversale_simple);
    println!("#Carre: {:?}, #experiments: {:?}, #probability: {:?}", num_carre, num, p_carre);
    println!("#Zero: {:?}, #experiments: {:?}, #probability: {:?}", num_zeros, num, p_zeros);
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
