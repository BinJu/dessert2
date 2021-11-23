use std::{collections::HashMap, fs::read_to_string, io};

use crate::{extract::OutputFormat, idents::{self, IdentObj}};

const TEMPLATE: &str = "template";
const TEMPLATE_FILE: &str = "template-file";
const OUTPUT_FORMAT: &str = "output-format";
const URL: &str = "url";

pub fn parse_params<I: Iterator<Item=String>>(params: I) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let mut key = String::new();
    for arg in params {
        if arg.starts_with("--") {
            if !key.is_empty() {
                result.insert(key, "".to_string());
            }
            key = String::from(&arg[2..]);
        } else {
            if key.is_empty() {
                println!("[WARN]: ignore the param without leading \"--\"");
                continue;
            }
            result.insert(key, arg);
            key = "".to_string();
        }
    }

    if !key.is_empty() {
        result.insert(key, "".to_string());
    }
    result
}

pub fn read_from_stdin() -> String {
    let mut buff = String::new();
    while let Ok(n) = io::stdin().read_line(&mut buff) {
        if n <= 0 {break};
    }
    buff
}

// return (type: json|yaml, content)
fn read_template_from_file(file_path: &String) -> (String, String) {
    let mut file_type = "yaml";
    if file_path.contains("yaml") || file_path.contains("yml") {
       file_type = "yaml"; 
    } else if file_path.contains("json") {
        file_type = "json";
    }
    (file_type.to_string(), read_to_string(&file_path).unwrap())
}

pub fn parse_output_format(params: &HashMap<String, String>) -> OutputFormat {
    if let Some(format_str) = params.get(&OUTPUT_FORMAT.to_owned()) {
        match &**format_str {
            "yaml" | "Yaml" | "YAML" => OutputFormat::Yaml,
            "json" | "Json" | "JSON" => OutputFormat::Json,
            "text"| "Text" | "TEXT" => OutputFormat::Text,
            _ => OutputFormat::Yaml

        }
    } else {
        OutputFormat::Yaml
    }
}

// read template
pub fn read_template(params: &HashMap<String, String>) -> Vec<IdentObj>{
    if params.contains_key(&TEMPLATE.to_owned()) {
        let template = params.get(&TEMPLATE.to_owned()).unwrap();
        if template.starts_with("---") {
            idents::from_yaml_str(template).unwrap()
        }
        else {
            idents::from_json_str(template).unwrap()
        }
    } else if params.contains_key(&TEMPLATE_FILE.to_owned()) {
        let file_name = params.get(&TEMPLATE_FILE.to_owned()).unwrap();
        let (file_type, file_content) = read_template_from_file(&file_name);
        if file_type == "json" {
            idents::from_json_str(&*file_content).unwrap()
        } else if file_type == "yaml" {
            idents::from_yaml_str(&*file_content).unwrap()
        } else {
            panic!("the file type should be either `json` or `yaml`");
        }
    } else {
        panic!("Either {} or {} must be specified", TEMPLATE, TEMPLATE_FILE);
    }
}

pub fn read_html(params: &HashMap<String, String>) -> String {
    if let Some(url) = params.get(&URL.to_string()){
       reqwest::blocking::get(url).unwrap().text().unwrap()
    } else {
       read_from_stdin() 
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_params_happy() {
        let params = vec!["--url".to_string(), "https://www.google.com".to_string(), "--output_format".to_string(), "json".to_string()];
        let parsed = parse_params(params.into_iter());
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed.get(&"output_format".to_owned()).unwrap(), &"json".to_owned());
        assert_eq!(parsed.get(&"url".to_owned()).unwrap(), &"https://www.google.com".to_owned());
    }

    #[test]
    fn parse_params_key_only() {
        let params = vec!["--text".to_string()];
        let parsed = parse_params(params.into_iter());
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed.get(&"text".to_owned()).unwrap(), &"".to_owned());
    }
}
