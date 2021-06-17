use iterator::garapon::Garapon;
use strum_macros::Display;

#[derive(Display)]
pub enum Ball {
    White,
    Green,
    Red,
    Blue,
    Yellow,
    Pink,
}

fn main() {
    // https://www.amazon.co.jp/dp/B00GXSKTNQ
    let mut garapon = Garapon::new();

    // 無色の玉を複数いれる
    for _ in 1..=5 {
        garapon.push(Ball::White);
    }

    // 色付きの玉を1個入れる
    for _ in 1..=1 {
        garapon.push(Ball::Green);
        garapon.push(Ball::Red);
        garapon.push(Ball::Blue);
        garapon.push(Ball::Yellow);
        garapon.push(Ball::Pink);
    }

    // 11 回してみる
    for _ in 1..=11 {
        match garapon.next() {
            Some(v) => println!("{}", v),
            None => println!("...empty!"),
        }
    }
}
