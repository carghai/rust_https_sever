use std::fs::{File, OpenOptions};
use std::io::{Error, Write};
use files::WriteData;
use crate::func::files;

impl WriteData {
    pub fn normal(&self, data: String, path: &str) -> Result<(), Error> {
        let output = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path);
        return WriteData {}.process(output, data);
    }
    pub fn replace(&self, data: String, path: &str) -> Result<(), Error> {
        let output = File::create(path);
        return WriteData {}.process(output, data);
    }
    fn process(&self, output: Result<File, Error>, data: String) -> Result<(), Error> {
        return match output {
            Ok(mut file) => {
                let error = write!(file, "{}\n", data);
                file.flush()?;
                match error {
                    Ok(..) => {
                        Ok(())
                    }
                    Err(e) => {
                        Err(e)
                    }
                }
            }
            Err(e) => {
                Err(e)
            }
        };
    }
}