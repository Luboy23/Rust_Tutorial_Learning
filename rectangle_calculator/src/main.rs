fn main() {
    let rect = Rectangle{
       width:30,
       length:50
    };
    println!("面积为：{}", area(&rect));
    println!("面积为：{:#?}", rect);


}

fn area(rect: &Rectangle) -> u32 {
    return rect.width * rect.length
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32
}

