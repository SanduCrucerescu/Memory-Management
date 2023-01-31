use crate::block::Operation;
use crate::memory_management;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, Write};
use std::{fs, io};

#[derive(Debug, Clone)]
pub struct FileApi {
    pub filename: String,
    pub out: i32,
}

impl FileApi {
    pub fn read_file(&self) -> (i32, Vec<Operation>) {
        let mut maxbytes = 0;
        let mut operations = Vec::new();
        let filename = format!("{}.in", self.filename);
        let file = fs::File::open(filename).expect("Unable to open file");
        let buff_reader = io::BufReader::new(file);

        let lines: Vec<String> = buff_reader
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .map(|l| l.to_string())
            .collect();

        for line in lines {
            if !line.starts_with("A")
                && !line.starts_with("D")
                && !line.starts_with("O")
                && !line.starts_with("C")
            {
                maxbytes = line.parse::<i32>().unwrap();
            } else {
                if line.starts_with("A") {
                    let l = line.split(";").collect::<Vec<&str>>();
                    let alloc = Operation {
                        id: operations.len() + 1,
                        bl_id: Some(l[1].parse::<i32>().unwrap()),
                        operation: l[0].chars().nth(0).unwrap(),
                        argument: Some(l[2].parse::<i32>().unwrap()),
                    };
                    operations.push(alloc);
                }
                if line.starts_with("D") {
                    let l = line.split(";").collect::<Vec<&str>>();
                    let dealloc = Operation {
                        id: operations.len() + 1,
                        bl_id: Some(l[1].parse::<i32>().unwrap()),
                        operation: l[0].chars().nth(0).unwrap(),
                        argument: Some(l[1].parse::<i32>().unwrap()),
                    };
                    operations.push(dealloc);
                }
                if line.starts_with("C") {
                    let l = line.split(";").collect::<Vec<&str>>();
                    let compress = Operation {
                        id: operations.len() + 1,
                        bl_id: None,
                        operation: l[0].chars().nth(0).unwrap(),
                        argument: None,
                    };
                    operations.push(compress);
                }
                if line.starts_with("O") {
                    let l = line.split(";").collect::<Vec<&str>>();
                    let output = Operation {
                        id: operations.len() + 1,
                        bl_id: None,
                        operation: l[0].chars().nth(0).unwrap(),
                        argument: None,
                    };
                    operations.push(output);
                }
            }
        }

        (maxbytes, operations)
    }

    pub fn write_file(
        &self,
        int: bool,
        method: &str,
        fragmentation: f64,
        all: Vec<String>,
        free: Vec<String>,
        errors: Vec<memory_management::Result>,
    ) {
        let mut file: Result<File, std::io::Error>;
        if int {
            file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(format!("{}.out{}", self.filename, self.out));
            match file {
                Ok(ref mut f) => {
                    f.write_all("\n".as_bytes()).unwrap();
                }
                Err(_) => {
                    file = Ok(fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(format!("{}.out{}", self.filename, self.out))
                        .expect("Unable to open file"));
                }
            }
        } else {
            file = Ok(fs::OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(format!("{}.out", self.filename))
                .expect("Unable to open file"));
        }
        let mut str = self.buff(method, fragmentation, all, free, errors);
        file.unwrap()
            .write_all(str.as_bytes())
            .expect("Unable to write data");
    }

    pub fn buff(
        &self,
        method: &str,
        fragmentation: f64,
        all: Vec<String>,
        free: Vec<String>,
        errors: Vec<memory_management::Result>,
    ) -> String {
        let mut str = String::new();
        str.push_str(method);
        str.push_str("\nAllocated blocks\n");
        for block in all.clone() {
            str.push_str(&block);
            str.push_str("\n");
        }
        str.push_str("Free blocks\n");
        for block in free.clone() {
            str.push_str(&block);
            str.push_str("\n");
        }
        str.push_str("Fragmentation\n");
        str.push_str(format!("{:.6}", fragmentation).as_str());
        str.push_str("\nErrors\n");
        if errors.is_empty() {
            str.push_str("None\n");
        } else {
            for e in errors.clone() {
                str.push_str(&e.to_string());
                str.push_str("\n");
            }
        }
        str
    }

    pub fn clear_file(&self) {
        let filename = format!("{}.out", self.filename);
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)
            .expect("Unable to open file");
    }
}
