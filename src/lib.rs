use std::{error::Error, fs};

// 返回一个实现了 Error 的 trait 的类型
// dyn -> dynamic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &content) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            // panic!("Not enough arguments!");
            return Err("Not enough arguments!");
        }
        // 引用时就需要管理生命周期
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// 切片引用数据有效时切片本身得有效，这样可以保持生命周期相同
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = vec![];

    for line in content.lines() {
        if line.contains(query) {
            res.push(line)
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick Three";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
