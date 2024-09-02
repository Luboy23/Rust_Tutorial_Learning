use std::{cmp::Ordering, io};    // prelude
use rand::Rng;

fn main() {
    println!("这是一个猜数游戏！！!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("猜测一个数字!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("你猜测的数是:{}", guess);

        match guess.cmp(&secret_number) {
        Ordering::Less => println!("你猜的数字太小了！"),
        Ordering::Greater => println!("你猜的数字太大了！"),
        Ordering::Equal => {
            println!("你猜对数字了！");
            break;
            }
        }
    }
    println!("神秘数字是:{}", secret_number);
} 
