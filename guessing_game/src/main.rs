use std::io;    // prelude

fn main() {
    println!("这是一个猜数游戏！！!");

    println!("猜测一个数字!");

    // let mut foo = 1;

    // let bar = foo; // 不可变

    // foo = 2;

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜测的数是:{}", guess);

} 
