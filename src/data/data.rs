use serde_json::Value;
use std::collections::HashMap;
use std::fs;

pub struct Categories {
    categories: Vec<HashMap<String, Value>>,
}

pub struct Category {
    category: HashMap<String, Value>,
}

pub struct CategoryDetails {
    details: HashMap<String, Value>,
}

impl Categories {
    pub fn new() -> Categories {
        let mut ctg = Categories {
            categories: Vec::new(),
        };

        ctg.load_data();
        ctg
    }

    pub fn add_category(&mut self, category: HashMap<String, Value>) {
        self.categories.push(category);
    }

    pub fn get_categories(&self, count: i32) -> Vec<HashMap<String, Value>> {
        self.categories[..count as usize].to_vec()
    }

    pub fn get_category(&self, index: i32) -> &HashMap<String, Value> {
        &self.categories[index as usize]
    }

    fn load_data(&mut self) {
        let data = include_str!("../data/categories.json");
        let json: Vec<HashMap<String, Value>> = serde_json::from_str(data).unwrap();
        for category in json.iter() {
            self.add_category(category.clone());
        }
    }
}

impl CategoryDetails {
    pub fn new() -> CategoryDetails {
        let mut details = CategoryDetails {
            details: HashMap::new(),
        };

        details.load_data();
        details
    }

    pub fn get_details(&self, index: i32) -> &HashMap<String, Value> {
        &self.details
    }

    pub fn get_detail(&self, index: i32) -> &Value {
        &self.details[&index.to_string()]
    }

    fn load_data(&mut self) {
        let data = include_str!("../data/category_details.json");
        let json: HashMap<String, Value> = serde_json::from_str(data).unwrap();
        self.details = json;
    }
}

