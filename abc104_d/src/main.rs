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
        ($line:expr, $name:ident, $t:tt) => {
            let $name = value!($line, $t);
        };
    }

    macro_rules! values_def {
        ($lines:expr, $n:expr, $name:ident, $t:tt) => {
            let $name = {
                let mut vec = Vec::new();
                for i in 0..$n {
                    let mut next = $lines.next().unwrap().split_whitespace();
                    vec.push(value!(next, $t));
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Type {
    A,
    B,
    C,
    Q,
}

fn solve(input: String) -> String {
    input!(input=>(s:#));
    let mut list = s
        .chars()
        .map(|x| match x {
            'A' => Type::A,
            'B' => Type::B,
            'C' => Type::C,
            _ => Type::Q,
        })
        .collect::<Vec<_>>();

    list.reverse();

    let mut a = 0i64;
    let mut b = 0i64;
    let mut c = 3_i64.pow(list.clone().into_iter().filter(|&x| x == Type::Q).count() as u32);
    for x in list {
        match x {
            Type::A => {
                a += b;
            }
            Type::B => {
                b += c;
            }
            Type::C => {
                c += 1;
            }
            Type::Q => {
                a += b / 3;
                b += c / 3;
                c += 3;
            }
        }
    }
    a.to_string()
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
    test1: "A??C" => "8",
    test2: "ABCBC" => "3",
    test3: "????C?????B??????A???????" => "979596887",
}
