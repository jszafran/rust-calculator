use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize, Debug)]
enum ColumnType {
    Question,
    Demographic,
}



#[derive(Deserialize, Debug)]
struct SurveyColumn {
    code: String,
    text: String,
    min_value: u8,
    max_value: u8,
    nullable: bool,
    of_type: String,
}

#[derive(Deserialize, Debug)]
struct Schema {
    org_node_column: String,
    columns: Vec<SurveyColumn>,
}

fn parse_schema(path: String) -> Schema {
    let contents = match fs::read_to_string(path.clone()) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{path}`");
            String::from("fooo")
        }
    };

    let schema: Schema = match toml::from_str(&contents) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Unable to parse TOML from {path}");
            exit(1);
        }
    };
    schema
}

fn main() {
    let parsed_schema = parse_schema(String::from("data_examples/example_schema.toml"));
    println!("{:?}", parsed_schema)
}
