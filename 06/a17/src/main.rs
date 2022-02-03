fn main() {
    println!("Hello, world!");
    //let mut start_number: i32 = 6397;
    let mut start_number: i32 = 7304;
    for i in 0..80 {
        let tmp = start_number*start_number;
        let front = tmp / 100;
        let middle = front % 10000;
        println!("{:?}  {:?}  {:?}  {:?}  {:?}",i, start_number, tmp, front, middle);
        start_number = middle;
    }

}

