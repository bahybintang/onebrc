use std::collections::HashMap;
use std::io::{BufRead, BufReader};

struct City {
    min: f32,
    max: f32,
    avg: f32,
    cnt: f32,
}

fn read_lines(file_path: &str) -> std::io::Result<std::io::BufReader<std::fs::File>> {
    let file = std::fs::File::open(file_path)?;
    Ok(BufReader::new(file))
}

fn main() {
    let mut h_map: HashMap<String, City> = HashMap::new();
    let mut counter = 0;

    for line in read_lines("data/measurements.txt").unwrap().lines() {
        counter += 1;

        let line = line.unwrap();
        let parts = line.split(';').collect::<Vec<&str>>();
        let city_name = parts[0];
        let temp = parts[1].parse::<f32>().unwrap();

        match h_map.get_mut(city_name) {
            Some(city) => {
                city.min = city.min.min(temp);
                city.max = city.max.max(temp);
                city.avg = (city.avg * city.cnt + temp) / (city.cnt + 1.0);
                city.cnt += 1.0;
            }
            None => {
                let city = City {
                    min: temp,
                    max: temp,
                    avg: temp,
                    cnt: 1.0,
                };
                h_map.insert(city_name.to_string(), city);
            }
        }

        if counter % 1_000_000 == 0 {
            println!("Processed {} lines", counter);
        }
    }

    for (city_name, city) in h_map.iter() {
        println!(
            "{}: min: {:.2}, max: {:.2}, avg: {:.2}",
            city_name, city.min, city.max, city.avg
        );
    }
}
