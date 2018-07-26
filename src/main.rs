#[derive(Debug)]
enum Fzv {
    Value(u64),
    Fizz(u64),
    Buzz(u64),
    FizzBuzz(u64),
}

fn main() {
    for n in (1..100).map(fzbzing) {
        println!("{:?}", n)
    }
}

fn fzbzing(n: u64) -> Fzv {
    match n {
        n if n % 15 == 0 => Fzv::FizzBuzz(n),
        n if n % 5 == 0 => Fzv::Buzz(n),
        n if n % 3 == 0 => Fzv::Fizz(n),
        n => Fzv::Value(n),
    }
}
