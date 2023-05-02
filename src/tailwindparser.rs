use std::collections::HashMap;

pub struct TailwindStrCont {
    str_map: HashMap<String, String>,
}

impl TailwindStrCont {
    pub fn new() -> Self {
        Self {
            str_map: HashMap::new(),
        }
    }
    pub fn add(&mut self, class: &str, tailstr: &str) {
        self.str_map.insert(class.to_owned(), tailstr.to_owned());
    }
    pub fn get(&self, class: &str) -> String {
        self.str_map.get(class).unwrap().to_owned()
    }
    pub fn pop(&mut self, class: &str) -> Option<(String, String)> {
        self.str_map.remove_entry(class)
    }
    // pub fn parse(&self, class: &str) -> Vec<&str> {
    //     self.str_map
    //         .get(class)
    //         .unwrap()
    //         .split_whitespace()
    //         .collect()
    // }
}

pub fn parse_tailstr(tailstr: String) -> Vec<String> {
    tailstr
        .split_whitespace()
        .map(|str| str.to_owned())
        .collect()
}
