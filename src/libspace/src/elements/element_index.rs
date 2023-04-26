use crate::elements::element_store::ElementStore;
use std::collections::HashMap;

pub struct ElementIndex {
    by_name: Vec<u64>,
    by_id: Vec<u64>,
    text_index: HashMap<String, Vec<u64>>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ElementSort {
    ByName,
    ById,
}

fn sat_name_split(input: &str) -> Vec<String> {
    input
        .to_uppercase()
        .clone()
        .split(&['-', '+', ' ', '=', '/'][..])
        .map(|x| x.to_owned())
        .collect()
}

impl ElementIndex {
    pub fn empty() -> Self {
        Self {
            by_name: vec![],
            by_id: vec![],
            text_index: HashMap::new(),
        }
    }

    pub fn from_store(store: &ElementStore) -> Self {
        let mut new_index = Self::empty();
        let elements = &store.elements;

        new_index.by_id = elements.keys().cloned().collect::<Vec<u64>>();
        let mut values = elements
            .iter()
            .map(|(id, element)| (element.object_name.clone().unwrap_or("".to_owned()), *id))
            .collect::<Vec<(String, u64)>>();
        values.sort_by(|(str, _id), (str2, _id2)| str.cmp(str2));

        for (name, id) in values {
            new_index.by_name.push(id);
            if let Some(e) = new_index.text_index.get_mut(&name) {
                e.push(id);
            } else {
                new_index.text_index.insert(name.clone(), vec![id]);
            }

            let split_name = sat_name_split(&name);
            for n in split_name {
                if !n.is_empty() && n != name {
                    if let Some(e) = new_index.text_index.get_mut(&n) {
                        e.push(id);
                    } else {
                        new_index.text_index.insert(n, vec![id]);
                    }
                }
            }
        }

        new_index
    }

    pub fn get_by(&self, by: ElementSort) -> &Vec<u64> {
        match by {
            ElementSort::ByName => &self.by_name,
            ElementSort::ById => &self.by_id,
        }
    }

    pub fn find_str(&self, s: &str, exact_match: bool) -> Vec<u64> {
        let words = if exact_match {
            vec![s.to_string()]
        } else {
            sat_name_split(s)
        };

        let mut result = Vec::new();
        for word in words {
            if let Some(ids) = self.text_index.get(&word) {
                result.extend(ids.iter());
            }
        }
        result
    }
}
