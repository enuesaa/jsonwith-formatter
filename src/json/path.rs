#[derive(Debug, Clone, PartialEq)]
pub struct JsonPathIndicator {
    pub indicate: String,
    pub count: usize,
}
impl JsonPathIndicator {
    fn add_count(&mut self) {
        self.count += 1;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub value: Vec<String>, // [".", ".aaa", "[]", "[0]."]
    pub indicators: Vec<JsonPathIndicator>,
}
impl Path {
    pub fn new() -> Self {
        Path {
            value: Vec::new(),
            indicators: Vec::new(),
        }
    }

    pub fn start_dict(&mut self) {
        self.add_list_key_if_in_list_scope();
        self.indicators.push(JsonPathIndicator {
            indicate: String::from("dict"),
            count: 0,
        });
        self.value.push(".".to_string());
    }

    pub fn add_dict_key(&mut self, value: String) {
        if let Some(last) = self.indicators.last_mut() {
            last.add_count();
        }
        if let Some(last_value) = self.value.last_mut() {
            *last_value = String::from(".") + &value;
        }
    }

    pub fn end_dict(&mut self) {
        self.value.pop();
        self.indicators.pop();
    }

    pub fn start_list(&mut self) {
        self.indicators.push(JsonPathIndicator {
            indicate: String::from("list"),
            count: 0,
        });
        self.value.push(String::from("[]"));
    }

    pub fn add_list_key_if_in_list_scope(&mut self) {
        if let Some(last) = self.indicators.last_mut() {
            if last.indicate == *"list" {
                if let Some(last_value) = self.value.last_mut() {
                    *last_value = String::from("[") + &last.count.to_string() + "]";
                }
                last.add_count();
            }
        }
    }

    pub fn end_list(&mut self) {
        self.value.pop();
        self.indicators.pop();
    }

    pub fn to_string(&mut self) -> String {
        self.value.clone().join("")
    }

    pub fn is_root(&mut self) -> bool {
        self.indicators.len() == 1
    }

    pub fn get_last_indicator(&mut self) -> JsonPathIndicator {
        self.indicators.last().unwrap().clone()
    }

    pub fn get_previous_indicator(&mut self) -> JsonPathIndicator {
        self.indicators[self.indicators.len() - 2].clone()
    }
}
