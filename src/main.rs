use std::collections::HashMap;
use std::fs::read_to_string;

struct City {
    min: f32,
    max: f32,
    avg: f32,
    cnt: f32,
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    let contents = read_to_string(filename).expect("Something went wrong reading the file");
    for line in contents.lines() {
        lines.push(line.to_string());
    }

    lines
}

fn main() {
    let lines = read_lines("data/measurements.txt");
    let mut h_map: HashMap<String, City> = HashMap::new();

    for line in lines {
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
    }

    for (city_name, city) in h_map.iter() {
        println!(
            "{}: min: {:.2}, max: {:.2}, avg: {:.2}",
            city_name, city.min, city.max, city.avg
        );
    }
}
