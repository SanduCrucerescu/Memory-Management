use crate::block::Operation;
use crate::memory_management;
use std::fs::OpenOptions;
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
                        bl_id: Some(l[1].parse::<i32>().unwrap()),
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
        let mut filename = String::new();
        if int {
            filename = format!("{}.out{}", self.filename, self.out);
        } else {
            filename = format!("{}.out", self.filename);
        }
        //let mut file = fs::File::create(filename).expect("Unable to create file");
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(filename)
            .expect("Unable to open file");
        let temp = vec![
            "\nAllocated blocks",
            "Free blocks",
            "Fragmentation",
            "\nErrors",
        ];
        file.write_all(method.as_bytes())
            .expect("Unable to write data");
        for (i, t) in temp.iter().enumerate() {
            file.write_all(t.as_bytes()).expect("Unable to write data");
            file.write_all("\n".as_bytes())
                .expect("Unable to write data");
            if i == 0 {
                for block in all.clone() {
                    let b = format!("{}\n", block);
                    file.write_all(b.as_bytes()).expect("Unable to write data");
                }
            } else if i == 1 {
                for block in free.clone() {
                    let b = format!("{}\n", block);
                    file.write_all(b.as_bytes()).expect("Unable to write data");
                }
            } else if i == 2 {
                let frag = format!("{:.6}", fragmentation);
                file.write_all(frag.as_bytes())
                    .expect("Unable to write data");
            } else if i == 3 {
                let mut err = String::new();
                if errors.is_empty() {
                    err = format!("{}", "None");
                    file.write_all(err.as_bytes())
                        .expect("Unable to write data");
                } else {
                    for e in errors.clone() {
                        err = format!("{}\n", e);
                        file.write_all(err.as_bytes())
                            .expect("Unable to write data");
                    }
                }
            }
        }
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
