use memmap::{Mmap, MmapOptions};
use rayon::prelude::*;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

struct City {
    min: f64,
    max: f64,
    sum: f64,
    cnt: u32,
}

impl City {
    fn new() -> City {
        City {
            min: f64::MAX,
            max: f64::MIN,
            sum: 0.0,
            cnt: 0,
        }
    }

    fn update(&mut self, temp: f64) {
        self.min = self.min.min(temp);
        self.max = self.max.max(temp);
        self.sum += temp;
        self.cnt += 1;
    }
}

fn read_lines(file_path: &str) -> std::io::Result<std::io::BufReader<std::fs::File>> {
    let file = std::fs::File::open(file_path)?;
    Ok(BufReader::new(file))
}

fn read_lines_mmap(file_path: &str) -> Mmap {
    let file = std::fs::File::open(file_path).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    mmap
}

fn sol_2() {
    let cores = std::thread::available_parallelism().unwrap().into();
    let mmap = read_lines_mmap("data/measurements.txt");

    let chunk_size = mmap.len() / cores;
    let mut chunks: Vec<(usize, usize)> = Vec::new();
    let mut start: usize = 0;

    for _ in 0..cores {
        let end = (start + chunk_size).min(mmap.len());
        let next_newline = match memchr::memchr(b'\n', &mmap[end..]) {
            Some(v) => v,
            None => {
                assert_eq!(end, mmap.len());
                0
            }
        };
        chunks.push((start, end + next_newline));
        start = end + next_newline + 1;
    }

    let results: Vec<_> = chunks
        .par_iter()
        .map(|(start, end)| {
            let mut h_map: HashMap<String, City> = HashMap::new();
            let mut counter = 0;

            for line in mmap[*start..*end].lines() {
                counter += 1;

                let line = line.unwrap();
                let parts = line.split(';').collect::<Vec<&str>>();
                let city_name = parts[0];
                let temp = parts[1].parse::<f64>().unwrap();

                match h_map.get_mut(city_name) {
                    Some(city) => {
                        city.update(temp);
                    }
                    None => {
                        h_map.insert(city_name.to_string(), City::new());
                    }
                }

                if counter % 1_000_000 == 0 {
                    println!("Processed {} lines in chunk {}..{}", counter, start, end);
                }
            }

            h_map
        })
        .collect();

    let mut final_map: HashMap<String, City> = HashMap::new();
    for h_map in results {
        for (city_name, city) in h_map.iter() {
            match final_map.get_mut(city_name) {
                Some(final_city) => {
                    final_city.min = final_city.min.min(city.min);
                    final_city.max = final_city.max.max(city.max);
                    final_city.sum += city.sum;
                    final_city.cnt += city.cnt;
                }
                None => {
                    final_map.insert(city_name.to_string(), City::new());
                }
            }
        }
    }

    for (city_name, city) in final_map.iter() {
        println!(
            "{}: min: {:.2}, max: {:.2}, avg: {:.2}",
            city_name,
            city.min,
            city.max,
            city.sum / city.cnt as f64
        );
    }
}

#[allow(dead_code)]
fn sol_1() {
    let mut h_map: HashMap<String, City> = HashMap::new();
    let mut counter = 0;

    for line in read_lines("data/measurements.txt").unwrap().lines() {
        counter += 1;

        let line = line.unwrap();
        let parts = line.split(';').collect::<Vec<&str>>();
        let city_name = parts[0];
        let temp = parts[1].parse::<f64>().unwrap();

        match h_map.get_mut(city_name) {
            Some(city) => {
                city.update(temp);
            }
            None => {
                h_map.insert(city_name.to_string(), City::new());
            }
        }

        if counter % 1_000_000 == 0 {
            println!("Processed {} lines", counter);
        }
    }

    for (city_name, city) in h_map.iter() {
        println!(
            "{}: min: {:.2}, max: {:.2}, avg: {:.2}",
            city_name,
            city.min,
            city.max,
            city.sum / city.cnt as f64
        );
    }
}

fn main() {
    // sol_1()
    sol_2()
}
