use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

pub fn get_out_dir() -> PathBuf {
    PathBuf::from(env::var_os("OUT_DIR").unwrap())
}

struct ParsedKepler {
    a: f64,
    a_cy: f64,
    e: f64,
    e_cy: f64,
    i: f64,
    i_cy: f64,
    l: f64,
    l_cy: f64,
    lp: f64,
    lp_cy: f64,
    ln: f64,
    ln_cy: f64,
    b: f64,
    c: f64,
    s: f64,
    f: f64,
}

impl ParsedKepler {
    fn empty() -> Self {
        Self {
            a: 0.0,
            a_cy: 0.0,
            e: 0.0,
            e_cy: 0.0,
            i: 0.0,
            i_cy: 0.0,
            l: 0.0,
            l_cy: 0.0,
            lp: 0.0,
            lp_cy: 0.0,
            ln: 0.0,
            ln_cy: 0.0,
            b: 0.0,
            c: 0.0,
            s: 0.0,
            f: 0.0,
        }
    }

    fn to_string(&self, name: &str) -> String {
        format!(
            r#"
            pub const {}: KeplerianElements = KeplerianElements {{
                semi_mayor_0: {:e},
                semi_mayor_cy: {:e},
                eccentricity_0: {:e},
                eccentricity_cy: {:e},
                inclination_0: {:e},
                inclination_cy: {:e},
                mean_longitude_0: {:e},
                mean_longitude_cy: {:e},
                long_perihelion_0: {:e},
                long_perihelion_cy: {:e},
                long_ascending_0: {:e},
                long_ascending_cy: {:e},
                b: {:e},
                c: {:e},
                s: {:e},
                f: {:e},
}};
        "#,
            name,
            self.a,
            self.a_cy,
            self.e,
            self.e_cy,
            self.i,
            self.i_cy,
            self.l,
            self.l_cy,
            self.lp,
            self.lp_cy,
            self.ln,
            self.ln_cy,
            self.b,
            self.c,
            self.s,
            self.f
        )
    }
}

fn load_base(filename: &PathBuf) -> HashMap<String, ParsedKepler> {
    let mut result = HashMap::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    enum ParserMode {
        SearchStart,
        FoundStart,
        FoundName,
    }

    let mut mode = ParserMode::SearchStart;
    let mut curr_name = String::new();
    'line_loop: for line in reader.lines() {
        let line = line.unwrap();
        match mode {
            ParserMode::SearchStart => {
                if line[0..4] == "----".to_string() {
                    mode = ParserMode::FoundStart;
                }
            }
            ParserMode::FoundStart => {
                if line[0..4] == "----".to_string() {
                    break 'line_loop;
                }
                curr_name = line[0..8].trim().to_string();
                let mut new_kepler = ParsedKepler::empty();
                let number_strings = line[8..line.len()]
                    .split_whitespace()
                    .collect::<Vec<&str>>();
                if number_strings.len() != 6 {
                    panic!(
                        "Kepler file is broken! Foudn {} elements?",
                        number_strings.len()
                    )
                }
                new_kepler.a = number_strings[0].parse().unwrap();
                new_kepler.e = number_strings[1].parse().unwrap();
                new_kepler.i = number_strings[2].parse().unwrap();
                new_kepler.l = number_strings[3].parse().unwrap();
                new_kepler.lp = number_strings[4].parse().unwrap();
                new_kepler.ln = number_strings[5].parse().unwrap();

                result.insert(curr_name.clone(), new_kepler);
                mode = ParserMode::FoundName;
            }
            ParserMode::FoundName => {
                if line[0..4] == "----".to_string() {
                    break 'line_loop;
                }
                let number_strings = line[10..line.len()]
                    .split_whitespace()
                    .collect::<Vec<&str>>();
                if number_strings.len() != 6 {
                    panic!("Kepler file is broken!")
                }

                let new_kepler = result.get_mut(&curr_name.clone()).unwrap();
                new_kepler.a_cy = number_strings[0].parse().unwrap();
                new_kepler.e_cy = number_strings[1].parse().unwrap();
                new_kepler.i_cy = number_strings[2].parse().unwrap();
                new_kepler.l_cy = number_strings[3].parse().unwrap();
                new_kepler.lp_cy = number_strings[4].parse().unwrap();
                new_kepler.ln_cy = number_strings[5].parse().unwrap();
                mode = ParserMode::FoundStart;
            }
        }
    }
    result
}

fn load_bcsf(filename: &PathBuf, elements: &mut HashMap<String, ParsedKepler>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    enum ParserMode {
        SearchStart,
        FoundStart,
    }
    let mut mode = ParserMode::SearchStart;
    'line_loop: for line in reader.lines() {
        let line = line.unwrap();
        match mode {
            ParserMode::SearchStart => {
                if line[0..4] == "----".to_string() {
                    mode = ParserMode::FoundStart;
                }
            }
            ParserMode::FoundStart => {
                if line[0..4] == "----".to_string() {
                    break 'line_loop;
                }
                let curr_name = line[0..8].trim().to_string();
                let new_kepler = elements.get_mut(&curr_name).unwrap();
                let number_strings = line[8..line.len()]
                    .split_whitespace()
                    .collect::<Vec<&str>>();
                if number_strings.len() != 4 {
                    panic!(
                        "Kepler 2b file is broken! Foudn {} elements?",
                        number_strings.len()
                    )
                }
                new_kepler.b = number_strings[0].parse().unwrap();
                new_kepler.c = number_strings[1].parse().unwrap();
                new_kepler.s = number_strings[2].parse().unwrap();
                new_kepler.f = number_strings[3].parse().unwrap();
            }
        }
    }
}

fn make_name(name: &str, suffix: &str) -> String {
    let upper_name = name.to_ascii_uppercase();
    let var_name = if upper_name == "EM BARY" {
        "EARTH".to_owned() + "_" + &suffix
    } else {
        upper_name + "_" + &suffix
    };

    var_name
}

fn write_elements(filename: &PathBuf, elements: &HashMap<String, ParsedKepler>, suffix: &str) {
    println!("Outputting kepler to {:?}", filename);
    let mut out_file = File::create(filename).unwrap();
    for (name, element) in elements {
        out_file
            .write(element.to_string(&make_name(&name, suffix)).as_bytes())
            .unwrap();
    }
}

fn write_orbit_objects(filename: &PathBuf, elements: &HashMap<String, ParsedKepler>) {
    let mut out_file = File::create(filename).unwrap();
    for (name, _element) in elements {
        let orbit_name = make_name(&name, "ORBIT");
        let long_name = make_name(&name, "LONG");
        let short_name = make_name(&name, "SHORT");
        out_file
            .write(
                format!(
                    r#"
const {}: Orbit = Orbit {{
    elements_short: {},
    elements_long: {},
}};
            "#,
                    orbit_name, short_name, long_name
                )
                .as_bytes(),
            )
            .unwrap();
    }
}

pub fn gen_kepler() {
    let file_1 = Path::new("gen_sources/kepler_1.txt").to_path_buf();
    let out_file_1 = get_out_dir().join("kepler_short.rs");
    let short = load_base(&file_1);
    write_elements(&out_file_1, &short, "SHORT");

    let file_2a = Path::new("gen_sources/kepler_2a.txt").to_path_buf();
    let file_2b = Path::new("gen_sources/kepler_2b.txt").to_path_buf();
    let out_file_2 = get_out_dir().join("kepler_long.rs");

    let mut long = load_base(&file_2a);
    load_bcsf(&file_2b, &mut long);
    write_elements(&out_file_2, &long, "LONG");

    let out_orbit_objects = get_out_dir().join("kepler_orbits.rs");
    write_orbit_objects(&out_orbit_objects, &long);
}
