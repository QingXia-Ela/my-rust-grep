use std::{error::Error, fs};

// 返回一个实现了 Error 的 trait 的类型
// dyn -> dynamic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    println!("text is:\n{}", content);
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
