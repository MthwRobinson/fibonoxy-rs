use std::env;

use fibonoxy::emoji;
use fibonoxy::fibonacci;

fn main() {
    let args: Vec<String> = env::args().collect();

    let emoji_name = &args[1].to_string();
    let emoji = emoji::name_to_emoji(emoji_name);
    let emoji_code = emoji::emoji_code(emoji);

    let n: &usize = &args[2].to_string().parse::<usize>().unwrap();
    let max_fibo = n.clone();

    for i in 0..=max_fibo {
        let fibo_n = fibonacci::fibonacci_number(i);
        println!("{}", emoji_code.repeat(fibo_n));
    }
}
