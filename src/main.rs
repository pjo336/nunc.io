extern crate yaml_rust;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;

fn main() {
    // Expect first given arg is location of config yaml file
    let args: Vec<_> = env::args().collect();
    let mut f = File::open(&args[1]).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let config = load_config_yml(&s);
    println!("{:?}", config);
}

fn load_config_yml(location: &str) -> Option<yaml_rust::Yaml> {
    YamlLoader::load_from_str(location)
        .unwrap()
        .into_iter()
        .nth(0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_config_yml_test() {
        let s =
            "
                hosts:
                    - nuncio.com
                    - port: 1331
            ";
        if let Some(res) = load_config_yml(&s) {
            println!("{:?}", res); // debug print
            assert_eq!(res["hosts"][0].as_str().unwrap(), "nuncio.com");
            assert_eq!(res["hosts"][1]["port"].as_i64().unwrap(), 1331);
        }
    }
}