extern crate core;

use std::io::{self, Read};

#[macro_use]
mod parser {
    macro_rules! input {
    ($s:expr=>$($t:tt)*) => {
        let mut lines=$s.split("\n");
        $(
            line_parse!(lines,$t);
        )*
    };
    }

    macro_rules! line_parse {
    ($lines:expr,($($name:ident:$t:tt)*)) => {
        let mut line=$lines.next().unwrap().split_whitespace();
        $(value_def!(line,$name,$t);)*
    };

    //複数行
    ($lines:expr,{$n:expr;$name:ident:$t:tt}) => {
        values_def!($lines,$n,$name,$t);
    };
    }

    macro_rules! value_def {
    ($line:expr,$name:ident,$t:tt) => {
        let $name=value!($line,$t);
    };
    }

    macro_rules! values_def {
    ($lines:expr,$n:expr,$name:ident,$t:tt) => {
        let $name={
        let mut vec=Vec::new();
        for i in 0..$n{
            let mut next=$lines.next().unwrap().split_whitespace();
            vec.push(value!(next,$t));
        }
        vec
        };
    };
    }

    macro_rules! value {
    //配列
    ($line:expr,[$t:tt]) => {
        $line.map(|x|{
        let mut iter=::std::iter::once(x);
        value!(iter,$t)
        }).collect::<Vec<_>>()
    };
    //タプル
    ($line:expr,($($t:tt),*)) => {
        ($(value!($line,$t),)*)
    };
    //文字列
    ($line:expr,#) => {
        $line.next().unwrap()
    };
    //単一値
    ($line:expr,$t:ty) => {
        $line.next().unwrap().parse::<$t>().unwrap()
    };
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let output = solve(input.trim().to_string());
    println!("{}", output);
}

fn solve(input: String) -> String {
    input!(input=>(n:i64));
    let (_, is_prime) = prime_sieve(n as usize);
    let mut count = 0;
    for r in 2..(n + 1) {
        if is_prime[r as usize] {
            count = add(count, ncr(n, r));
        }
    }
    count.to_string()
}

//max以下の素数列挙
pub fn prime_sieve(n: usize) -> (Vec<usize>, Vec<bool>) {
    let mut prime = Vec::new();
    let mut is_prime = Vec::with_capacity(n + 1);
    is_prime.resize(n + 1, true);
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..n + 1 {
        if is_prime[i] {
            prime.push(i);
            {
                let mut j = 2 * i;
                while j <= n {
                    is_prime[j] = false;
                    j += i;
                }
            }
        }
    }

    (prime, is_prime)
}

fn add(a: i64, b: i64) -> i64 {
    (a + b) % 1000000007
}

fn mul(a: i64, b: i64) -> i64 {
    ((a % 1000000007) * (b % 1000000007)) % 1000000007
}

fn ncr(n: i64, r: i64) -> i64 {
    if r == 0 {
        1
    } else {
        mul((n - r + 1), ncr(n, r - 1)) / r
    }
}

macro_rules! tests {
    ($($name:ident: $input:expr=>$output:expr,)*) => {
        mod tests {
            $(
                #[test]
                fn $name() {
                    assert_eq!($output.trim().to_string(),super::solve($input.trim().to_string()));
                }
            )*
        }
    }
}

tests! {
    test1: "3" => "4",
    test2: "6" => "30",
    test3: "1000" => "972926972",
}