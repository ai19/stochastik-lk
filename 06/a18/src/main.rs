use std::env;

fn get_args(args: Vec<String>) -> (i32, i32, f64) {
    let raw_num_blocks:&String = &args[1];
    let raw_frac:&String = &args[2];
    let raw_start:&String = &args[3];
    let frac:i32 = raw_frac.parse::<i32>().unwrap();
    let start:f64 = raw_start.parse::<f64>().unwrap();
    let num_blocks:i32 = raw_num_blocks.parse::<i32>().unwrap();
    (num_blocks, frac, start)
}

fn _all_checked(checks: &Vec<bool>) -> bool {
    let mut result:bool = true;
    for check in checks {
        if !check {
            result = false;
        }
    }
    return result;
}

fn sammler_test(digits: &Vec<i32>) -> f64 {
    let mut sum_group_lengths: u32 = 0;
    let mut num_groups: u32 = 0;
    let mut num_digits_in_current_group: u32 = 0;
//    let mut digits_checked: u32 = 0;
//    let checks:Vec<u32> = vec![1,2,4,8,16,32,64,128,256,512,1024];
//    let all_checked: u32 = 1+2+4+8+16+32+64+128+256+512+1024;
    let mut checks:Vec<bool> = vec![false, false,false,false,false,false,false,false,false,false];

    for digit in digits {
        //digits_checked = digits_checked | checks[*digit as usize];
        checks[*digit as usize] = true;
        num_digits_in_current_group +=1;
        if _all_checked(&checks) {
            sum_group_lengths += num_digits_in_current_group;
            num_digits_in_current_group = 0;
 //           digits_checked = 0;
            num_groups += 1;
            checks = vec![false, false,false,false,false,false,false,false,false,false];
        } //else {
          //  num_digits_in_current_group += 1;
       // }
    }
    println!("sum_group_lenghts: {:?}   num_groups: {:?}   ", sum_group_lengths, num_groups);
    (sum_group_lengths as f64) / (num_groups as f64)
}


fn ziffern_auszaehl_test(digits: &Vec<i32>) -> Vec<f64> {
    let mut num_digit:Vec<i32> = vec![0,0,0,0,0,0,0,0,0,0];

    for digit in digits {
        num_digit[*digit as usize] += 1;
    }

    let mut result = Vec::<f64>::new();
    for i in 0..10 {
        result.push( num_digit[i] as f64 / digits.len() as f64);
    }

    result

}

fn maximum_test(digits: &Vec<i32>) -> f64 {
    let mut offset: i32 = 0;
    let mut num_middle_max = 0;
    let mut num_groups = 0;

    while offset < digits.len() as i32 - 3 {
        let left = digits[offset as usize];
        let middle = digits[(offset+1) as usize];
        let right = digits[(offset+2) as usize];
        if middle > left && middle > right {
            num_middle_max += 1;
        }
        offset += 3;
        num_groups += 1;
    }
    println!("Offset: {:?}, Length-3: {:?}   Groups: {:?}", offset, digits.len()-3, num_groups);

    return num_middle_max as f64 / num_groups as f64;

}


fn get_random_digit_vector(num_blocks: i32, frac_: i32, start: f64) -> Vec::<i32> {
    let mut result:Vec<i32> = Vec::<i32>::new();
    let mut xn:f64 = start;
    let mut xn1:f64; // = 0.0;
    let frac:f64 = frac_ as f64;

    for _ in 0..num_blocks {
        xn1 = xn * frac;
        xn = xn1 - xn1.floor();
//        println!("{:?} {:?}", xn1, xn);

        let mut tmp = (xn * 10000.0).floor() as i32;
        let mut tmp1 = tmp / 1000;
        result.push(tmp1);
        tmp -= 1000*tmp1;

        tmp1 = tmp / 100;
        result.push(tmp1);
        tmp -= 100*tmp1;

        tmp1 = tmp/10;
        result.push(tmp1);
        tmp -= 10*tmp1;
        result.push(tmp);

  //      println!("{:?} ", result);



//        println!("{:?}   {:?}  ", tmp, tmp/1000);
    }
    result
}


fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let (num_blocks, frac, start) = get_args(args);
    println!("{:?}   {:?}   {:?}", num_blocks, frac, start);
    let vec = get_random_digit_vector(num_blocks, frac, start);
    println!("Ziffern-Auszähl-Test:"); // {:?}", ziffern_auszaehl_test(&vec));
    let ziffern = ziffern_auszaehl_test(&vec);
    for i in 0..ziffern.len() {
        println!("{:?}    {:?}", i, ziffern[i as usize]);
    }
    println!("Sammler-Test: {:?}", sammler_test(&vec));
    println!("Maximum-Test: {:?}", maximum_test(&vec));


}
