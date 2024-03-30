use serde_json::Value;
use std::collections::HashMap;

pub struct Categories {
    categories: Vec<HashMap<String, Value>>,
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
        log::info!("data.rs::new - Categories: {:?}", ctg.categories.len());
        ctg
    }

    pub fn add_category(&mut self, category: HashMap<String, Value>) {
        self.categories.push(category);
    }

    pub fn get_categories(&self, count: i32) -> Vec<HashMap<String, Value>> {
        log::info!("data.rs::get_categories - count: {:?}", count);
        self.categories[..count as usize].to_vec()
    }

    pub fn get_category(&self, index: i32) -> &HashMap<String, Value> {
        log::info!("data.rs::get_category - index: {:?}", index);
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

    pub fn get_details(&self) -> &HashMap<String, Value> {
        &self.details
    }

    pub fn get_detail(&self, category_number: &str) -> &Value {
        log::info!("data.rs::get_detail - category_number: {:?}", category_number);
        &self.details[category_number]
    }

    fn load_data(&mut self) {
        let data = include_str!("../data/category_details.json");
        let json: HashMap<String, Value> = serde_json::from_str(data).unwrap();
        self.details = json;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categories() {
        let categories = Categories::new();
        assert_eq!(categories.get_categories(1).len(), 1);
        assert_eq!(categories.get_categories(2).len(), 2);
        assert_eq!(categories.get_categories(3).len(), 3);
    }

    #[test]
    fn test_category() {
        let categories = Categories::new();
        let category = categories.get_category(0);
        assert_eq!(category["id"], 2);
        assert_eq!(category["title"], "baseball");
    }
}