use csv::{Reader, ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "history.csv";

// TYPE, TAG, TEXT, LIFE
struct DataHistory {
    data_type: String,
    tag: String,
    text: String,
    life: i32,
}

impl DataHistory {
    fn new(row: StringRecord) -> DataHistory {
        return DataHistory {
            data_type: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            life: row.get(3).unwrap().trim().parse().unwrap_or(0),
        };
    }
}

fn main() {
    let mut data_history: HashMap<String, DataHistory> = HashMap::new();
    let content = fs::read_to_string(FILENAME).unwrap();

    // println!("{:?}", content);

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());
    for result in rdr.records() {
        let result = result.unwrap();

        let data = DataHistory::new(result);

        data_history.insert(data.tag.clone(), data);
    }

    let mut dict: HashMap<String, String> = HashMap::new();
    dict.insert("Apple".to_string(), "Red color fruit".to_string());

    println!("Description: {}", dict["Apple"]);
}
