use std::env;

use fibonoxy::emoji;
use fibonoxy::fibonacci;

fn main() {
    let args: Vec<String> = env::args().collect();

    let n: &i32 = &args[1].to_string().parse::<i32>().unwrap();
    let fibo_n = fibonacci::fibonacci_number(*n);

    let emoji_name = &args[2].to_string();
    let emoji = emoji::name_to_emoji(emoji_name);
    let emoji_code = emoji::emoji_code(emoji);

    println!("{}", emoji_code.repeat(fibo_n.try_into().unwrap()));
}
