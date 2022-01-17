use serde::{Deserialize, Serialize};

use sgp4::Elements;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::elements::element_store::ElementStore;
use crate::elements::element_util::*;
use crate::elements::ElementIndex;

pub struct ElementDb {
    element_store: ElementStore,
    index: ElementIndex,
}

fn init_dirs() -> Result<PathBuf, String> {
    if let Some(data_dir) = dirs::data_local_dir() {
        let our_dir = data_dir.join(Path::new("satwatch/"));
        if !our_dir.exists() {
            std::fs::create_dir_all(our_dir.clone()).unwrap();
            eprintln!("Creating {}", our_dir.to_str().unwrap());
        }

        Ok(our_dir)
    } else {
        Err("No idea where to save stuff? what?".to_string())
    }
}

impl ElementDb {
    pub fn new() -> Self {
        let data_dir = init_dirs().expect("Something is wrong, you cant write home!");
        let data_filename = data_dir.join(Path::new("elements.json"));
        if let Ok(file) = std::fs::File::open(data_filename) {
            if let Ok(store) = serde_json::from_reader(file) {
                return Self {
                    index: ElementIndex::from_store(&store),
                    element_store: store,
                };
            }
        }

        Self {
            element_store: ElementStore {
                elements: HashMap::new(),
            },
            index: ElementIndex::empty(),
        }
    }

    pub fn fetch_full_celestrak(&mut self) -> Result<(), String> {
        match ureq::get("https://celestrak.com/NORAD/elements/gp.php")
            .query("GROUP", "active")
            .query("FORMAT", "json")
            .call()
        {
            Ok(response) => match response.into_json::<Vec<Elements>>() {
                Ok(elements_group) => {
                    for e in elements_group {
                        self.element_store.elements.insert(e.norad_id, e);
                    }
                    Ok(())
                }
                Err(_) => Err("Deserialization error".to_string()),
            },
            Err(ureq::Error::Status(status, response)) => Err(format!(
                "Celestrak network error: {}: {}",
                status,
                response.into_string().unwrap()
            )),
            Err(_) => Err("Unknown network error".to_string()),
        }
    }

    pub fn save(&self) {
        let data_dir = init_dirs().expect("Something is wrong, you cant write home!");
        let data_filename = data_dir.join(Path::new("elements.json"));
        let f = File::create(data_filename).expect("Could not open elements file");
        serde_json::to_writer(f, &self.element_store).unwrap();
    }

    pub fn get(&self, key: u64) -> Option<&sgp4::Elements> {
        self.element_store.elements.get(&key)
    }

    pub fn get_copy(&self, key: u64) -> Option<sgp4::Elements> {
        if let Some(as_ref) = self.get(key) {
            Some(element_copy(as_ref))
        } else {
            None
        }
    }

    pub fn all(&self) -> &HashMap<u64, Elements> {
        return &self.element_store.elements;
    }

    pub fn index(&self) -> &ElementIndex {
        &self.index
    }
}
