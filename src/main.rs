use std::fs::{self, File};
use std::io::{Write, BufReader, BufRead, Error};
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Error> {
    let path = "areainfo/地区信息_中国地区_CN_EN.txt";

    let re = Regex::new(r"^(\w+)\s(Inner\sMongolia|Macao\sSAR|Hongkong\sSAR|\w+)\s([\w\s\\'\\-]+)$").unwrap();

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut hm_area:HashMap<String, HashMap<String, HashSet<String>>> = HashMap::new();

    for line in buffered.lines() {
        if let Ok(s) = line {
            if s.len() > 0 {
                for cap in re.captures_iter(s.as_str()) {
                    match hm_area.get_mut(&cap[1].to_string()) {
                        Some(province) => {
                            match province.get_mut(&cap[2].to_string()) {
                                Some(city) => {
                                    city.insert(cap[3].to_string());
                                },
                                None => {
                                    let mut city: HashSet<String> = HashSet::new();
                                    city.insert(cap[3].to_string());
                                    province.insert(cap[2].to_string(), city);
                                }
                            }
                        },
                        None => {
                            let mut city: HashSet<String> = HashSet::new();
                            let mut province: HashMap<String, HashSet<String>> = HashMap::new();
                            
                            city.insert(cap[3].to_string());
                            province.insert(cap[2].to_string(), city);
                            hm_area.insert(cap[1].to_string(), province);
                        }
                    }
                    // println!("Country: {} Province: {} City: {}", &cap[1], &cap[2], &cap[3]);
                }
            }
        }
    }

    fs::remove_file("dist/areainfo.js")?;
    let mut output = File::create("dist/areainfo.js")?;
    write!(output, "module.export = {{")?;
    for (country, provinces) in hm_area {
        let country_tag = if country.chars().take(1).next().unwrap().is_ascii_alphabetic() { "EN" } else { "CN" };
        write!(output, "{0}:[{{value:\"{1}\",label:\"{1}\",children:[", country_tag, country)?;
        for (province, citys) in provinces {
            write!(output,"{{value:\"{0}\",label:\"{0}\",children:[", province)?;
            for city in citys { 
                write!(output,"{{value:\"{0}\",label:\"{0}\"}},", city)?;
            }
            write!(output, "]}},")?;
        }
        write!(output, "]}}],")?;
    }
    write!(output, "}}")?;
    Ok(())
}