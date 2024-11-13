use csv::{Reader, ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

// TYPE, TAG, TEXT, LIFE
struct DataHistory {
    data_type: String,
    tag: String,
    text: String,
    life: i32,
    options: Vec<DataHistory>,
}

impl DataHistory {
    fn new(row: StringRecord) -> DataHistory {
        return DataHistory {
            data_type: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            life: row.get(3).unwrap().trim().parse().unwrap_or(0),
            options: vec![],
        };
    }
}

fn main() {
    let mut life = 100;
    let mut tag_actual = FIRST_TAG;

    let mut last_record: String = "".to_string();

    let mut data_history: HashMap<String, DataHistory> = HashMap::new();
    let content = fs::read_to_string(FILENAME).unwrap();

    // println!("{:?}", content);

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());
    for result in rdr.records() {
        let result = result.unwrap();
        let data = DataHistory::new(result);

        if data.data_type == "SITUACION" {
            let record_tag = data.tag.clone();
            data_history.insert(record_tag.clone(), data);
            last_record = record_tag;
        } else if data.data_type == "OPCION" {
            if let Some(new_data) = data_history.get_mut(&last_record) {
                (*new_data).options.push(data);
            }
        }
    }

    //Game Loop
    loop {
        println!("You have {} points of life", life);
        if let Some(new_data) = data_history.get(tag_actual) {
            println!("{}", new_data.text);

            for (index, option) in new_data.options.iter().enumerate() {
                println!("[{}] {}", index, option.text)
            }

            break;
        }
    }
}
