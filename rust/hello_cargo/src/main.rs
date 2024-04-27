use std::io;
use rand::Rng;
use futures::executor::block_on;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

async fn say_hello()->String {
    println!("hello");
    say_world().await;
    "hello".to_string()
}
async fn say_world() {
    println!("world");
}

fn main() {
    let op = say_hello();
    block_on(op);
    let c = 32.0;
    let f = f_to_c(c);
    println!("c:{c},f:{f}");
    let f = fibo(5);
    println!("fibo:{f}");
    song();
    let mut z = 0;
    println!("{z}");
    z = 1;
    println!("{z},{THREE_HOURS_IN_SECONDS}");
    // z = 0.1;
    let x = 3;
    let y = 5;
    // let x = 6;
    println!("x = {x} and y + 2 = {}",y + 2);
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{}--{}",secret_number,y);
    let mut guess = String::new();
    a_f('a',0);
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}
fn a_f(a:char,b:i32)->i32{
    let condition = true;
    let number = if condition { 5 } else { 6 };
    if condition {
        println!("{}",'5');
        // 5
    }else{
        println!("{}",'6');
        // 6
    }
    let y = {
        let x = 3;
        x + 1
    };
    let mut s = String::from("hello");
    s.push_str(", world!");
    let s1 = s;
    // s += "world";
    let s2 = "s2";
    let s3 = s2;
    println!("s2---s3:{s2}---{s3}");
    println!("a_f,{a},{b},{number},y:{y},{s1}");
    9
}
fn f_to_c(val:f32)->f32{
    32.0 + val * 1.8
}
fn fibo(n:u32)->u32{
    if n<3 {
        1
    }else{
        fibo(n-1) + fibo(n-2)
    }
}
fn song(){
    let arr = ["first","second","third"];
    let mut idx = 0;
    while idx < 3 {
        println!("On the {} day of Christmas my true love sent to me",arr[idx]);
        if idx == 0{
            println!("A partridge in a pear tree");
        }else if idx == 1 {
            println!("Two turtle doves,");
            println!("And a partridge in a pear tree.");
        }else {
            println!("Three French hens,");
            println!("Two turtle doves,");
            println!("And a partridge in a pear tree.");
        }
        idx+=1
    }
}