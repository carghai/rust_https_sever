use std::fs::File;
use std::io::{Write};
use std::{thread, time};

pub struct Data{}


impl Data {
    fn get(&self) {
        let mut x = 0;
        loop {
            x += 1;
            let  output = File::create("src/data_getting_test/cache.txt");
            match output {
                Ok(mut f) => {
                    let error = write!(f, "{}", x);
                    match error {
                        Ok(_) => {
                            thread::sleep(time::Duration::from_millis(1000));
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}