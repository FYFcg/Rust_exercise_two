use std::io::{self};

struct Buffer<T> {
    data: Vec<T>,
}

impl<T> Buffer<T>
where
    T: std::ops::Add<Output = T> + Default + Clone,
{
    fn new() -> Self {
        Buffer { data: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn sum(&self) -> T {
        self.data.iter().cloned().fold(T::default(), |acc, x| acc + x)
    }
}

fn main() {
    let mut buffer = Buffer::new();

    println!("请输入整数值,以空格分隔(输入q结束):");

    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).expect("读取输入时出错");
    let numbers: Vec<&str> = line.split_whitespace().collect();

    for number in numbers {
        if number == "q" {
            break;
        }

        if let Ok(number) = number.parse() {
            buffer.push(number);
        } else {
            println!("无效的输入");
        }
    }

    let sum: i32 = buffer.sum();
    println!("总和: {}", sum);
}
