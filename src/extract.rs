use crate::output::Output;
use crate::idents::{IdentObj, IdentityValueType, ValueFrom};
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize, Serializer};

use std::error::Error;
use std::fmt;
use std::collections::HashMap;
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;

/// Extract the text from the source(Html), return it as Output
///
pub struct Extractor<'a> {
    text: &'a str,
    output_format: OutputFormat
}

impl<'a> Extractor<'a> {
    pub fn new(source: &'a str, output_format: OutputFormat) -> Self {
        Self {
            text: source,
            output_format
        }
    }
    pub fn abstract_objs(&self, idents: &'a Vec<IdentObj>) -> Result<Output, ExtractError> {
        let intermediate_result = self.abstract_objs_intermediate(idents)?;
        intermediate_to_output(&intermediate_result, &self.output_format)
    }

    fn abstract_objs_intermediate(&self, idents: &'a Vec<IdentObj>) -> Result<IntermediateResult, ExtractError> {
        let mut result = IntermediateResult::new();
        let parsed = Html::parse_document(self.text);
        for ident in idents {
            let mut result_obj = IntermediateObject{ object_id: ident.object_id.clone(), records: Vec::new() };
            //locate the object by css selector
            let selector = Selector::parse(&*ident.css_selector)?;
            let mut dom_obj = parsed.select(&selector);
            while let Some(obj) = dom_obj.next() {
                let mut result_props = IntermediateProperty::new();
                for prop in &ident.properties {
                    let prop_selector = Selector::parse(&*prop.css_selector)?;
                    let mut dom_prop = obj.select(&prop_selector);
                    if let Some(v) = dom_prop.next() {
                        let mut selected_value = "".to_string();
                        let val = get_value_from_dom(&v, &prop.value_from);
                        if let Some(prop_value) = val { selected_value = prop_value };
                        result_props.insert(prop.id.clone(), convert_string_to_property_value(selected_value, &prop.value_type));
                    }
                }
                result_obj.records.push(result_props);
                
            }
            result.push(result_obj);
        }
        Ok(result)
    }
}

fn convert_string_to_property_value(value: String, prop_type: &IdentityValueType) -> PropertyValue {
    match prop_type {
        IdentityValueType::Str => PropertyValue::Str(value),
        IdentityValueType::Int => {
            let value: Result<i64, ParseIntError> = value.parse();
            match value {
                Ok(int_value) => PropertyValue::Int(int_value),
                Err(_)=> PropertyValue::NA,
            }
        },
        IdentityValueType::Float => {
            let value: Result<f64, ParseFloatError> = value.parse();
            match value {
                Ok(float_value) => PropertyValue::Float(float_value),
                Err(_)=> PropertyValue::NA,
            }
        },
        IdentityValueType::Bool => {
            let value: Result<bool, ParseBoolError> = value.parse();
            match value {
                Ok(bool_value) => PropertyValue::Bool(bool_value),
                Err(_)=> PropertyValue::NA,
            }
        }
    }
}

fn get_value_from_dom(elm_ref: &ElementRef, value_from: &ValueFrom) -> Option<String> {
    match value_from {
        ValueFrom::InnerText => Some(elm_ref.inner_html()),
        ValueFrom::Property(prop) => elm_ref.value().attr(&*prop).map(|prop_val|String::from(prop_val))
    } 
}

fn intermediate_to_output(intermediate: &IntermediateResult, output_format: &OutputFormat) -> Result<Output, ExtractError> {
    match output_format {
        OutputFormat::Json => Ok(Output::Json(serde_json::to_string(intermediate)?)),
        OutputFormat::Yaml => Ok(Output::Yaml(serde_yaml::to_string(intermediate)?)),
        OutputFormat::Text => {
            if intermediate.len() > 0 {
                let records = &intermediate[0].records;
                if records.len() > 0 {
                    let record = &records[0];
                    let value = record.values().next();
                    if let Some(v) = value {
                        Ok(Output::Text(format!("{}", v)))
                    } else {
                        Ok(Output::text(""))
                    }
                } else {
                    Ok(Output::text(""))
                }
            } else {
                Ok(Output::text(""))
            }
        }
    }
}

#[allow(dead_code)]
pub enum OutputFormat {
    Json,
    Yaml,
    Text
}

#[derive(Debug, PartialEq, Deserialize)]
enum PropertyValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    NA
}

impl Serialize for PropertyValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self {
            Self::Int(val) => serializer.serialize_i64(*val),
            Self::Float(val) => serializer.serialize_f64(*val),
            Self::Str(val) => serializer.serialize_str(val),
            Self::Bool(val) => serializer.serialize_bool(*val),
            Self::NA => serializer.serialize_none() 
        }
    }
}

impl fmt::Display for PropertyValue {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&*match &self {
            Self::Int(val) => val.to_string(),
            Self::Float(val) => val.to_string(),
            Self::Str(val) => val.clone(),
            Self::Bool(val) => val.to_string(),
            Self::NA => "".to_string()
        })?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum ExtractError {
    ParseSelector(String),
    SerdeJson(String),
    SerdeYaml(String),
}

impl Error for ExtractError {}

impl fmt::Display for ExtractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::ParseSelector(selector_error) => write!(f, "[Selector Error]: {}", selector_error),
            Self::SerdeJson(json_error) => write!(f, "SerdeJson Error: {}", json_error),
            Self::SerdeYaml(yaml_error) => write!(f, "SerdeYaml Error: {}", yaml_error),
        }
    }
}

impl From<cssparser::ParseError<'_, selectors::parser::SelectorParseErrorKind<'_>>> for ExtractError {
    fn from(item: cssparser::ParseError<'_, selectors::parser::SelectorParseErrorKind<'_>>) -> Self {
        ExtractError::ParseSelector(format!("[cssparser::ParseError]{:?}", item))
    }
}

impl From<serde_json::error::Error> for ExtractError {
    fn from(item: serde_json::error::Error) -> Self {
        ExtractError::SerdeJson(format!("[Json Error]: {}", item))
    }
}

impl From<serde_yaml::Error> for ExtractError {
    fn from(item: serde_yaml::Error) -> Self {
        ExtractError::SerdeYaml(format!("[Yaml Error]: {}", item))
    }
}
type IntermediateProperty = HashMap<String, PropertyValue>;
type IntermediateResult = Vec<IntermediateObject>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct IntermediateObject {
    object_id: String,
    records: Vec<IntermediateProperty>
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::idents::Identity;
    
    fn construct_ident_obj() -> IdentObj {
        IdentObj {
            object_id: "user-info".to_owned(),
            css_selector: "div#user_info".to_owned(),
            properties: vec![Identity {
                    id: "email".to_owned(),
                    css_selector: "div#email".to_owned(),
                    value_type: IdentityValueType::Str,
                    value_from: ValueFrom::InnerText
            }, Identity {
                    id: "phone-number".to_owned(),
                    css_selector: "div#phone_number".to_owned(),
                    value_type: IdentityValueType::Str,
                    value_from: ValueFrom::InnerText
            }]
        }
    }
    fn construct_ident_obj_single_prop() -> IdentObj {
        IdentObj {
            object_id: "user-info".to_owned(),
            css_selector: "div#user_info".to_owned(),
            properties: vec![Identity {
                    id: "email".to_owned(),
                    css_selector: "div#email".to_owned(),
                    value_type: IdentityValueType::Str,
                    value_from: ValueFrom::InnerText
            }]
        }
    }
    fn construct_multiple_ident_obj() -> Vec<IdentObj> {
        vec![IdentObj {
            object_id: "user-info".to_owned(),
            css_selector: "div#user_info".to_owned(),
            properties: vec![Identity {
                    id: "email".to_owned(),
                    css_selector: "div#email".to_owned(),
                    value_type: IdentityValueType::Str,
                    value_from: ValueFrom::InnerText
            }]
        },
        IdentObj {
            object_id: "book-info".to_owned(),
            css_selector: "div#book_info".to_owned(),
            properties: vec![Identity {
                    id: "isn".to_owned(),
                    css_selector: "div#isn".to_owned(),
                    value_type: IdentityValueType::Str,
                    value_from: ValueFrom::InnerText
            }]
        }]
    }

    #[test]
    fn convert_string_to_property_string_value() {
        let v = convert_string_to_property_value("simple text".to_string(), &IdentityValueType::Str);
        assert_eq!(v, PropertyValue::Str("simple text".to_string()));
    }

    #[test]
    fn convert_string_to_property_int_value() {
        let v = convert_string_to_property_value("1234567890".to_string(), &IdentityValueType::Int);
        assert_eq!(v, PropertyValue::Int(1234567890));
    }

    #[test]
    fn convert_string_to_property_float_value() {
        let v = convert_string_to_property_value("1234567890.1234".to_string(), &IdentityValueType::Float);
        assert_eq!(v, PropertyValue::Float(1234567890.1234));
    }

    #[test]
    fn convert_string_to_property_bool_true_value() {
        let v = convert_string_to_property_value("true".to_string(), &IdentityValueType::Bool);
        assert_eq!(v, PropertyValue::Bool(true));
    }

    #[test]
    fn convert_string_to_property_bool_false_value() {
        let v = convert_string_to_property_value("false".to_string(), &IdentityValueType::Bool);
        assert_eq!(v, PropertyValue::Bool(false));
    }

    #[test]
    fn convert_string_to_property_na_value() {
        let v = convert_string_to_property_value("simple text".to_string(), &IdentityValueType::Float);
        assert_eq!(v, PropertyValue::NA);
    }

    #[test]
    fn abstract_intermediate() {
        let ids = vec![construct_ident_obj()];
        let extractor = Extractor { text: "<html><head></head><body><div id=\"user_info\"><div id=\"email\">abc@abc.com</div><div id=\"phone_number\">13344445555</div></div></body></html>", output_format: OutputFormat::Yaml};
        let result = extractor.abstract_objs_intermediate(&ids).unwrap();

        let mut expected_props = HashMap::new();
        expected_props.insert("email".to_string(), PropertyValue::Str("abc@abc.com".to_string()));
        expected_props.insert("phone-number".to_string(), PropertyValue::Str("13344445555".to_string()));
        let expected_output = vec![ IntermediateObject {object_id: "user-info".to_string(), records: vec![expected_props]}];
        assert_eq!(result, expected_output);
    }

    #[test]
    fn abstract_intermediate_multiple_records() {
        let ids = vec![construct_ident_obj()];
        let extractor = Extractor { text: "<html><head></head><body><div id=\"user_info\"><div id=\"email\">abc@abc.com</div><div id=\"phone_number\">13344445555</div></div><div id=\"user_info\"><div id=\"email\">def@abc.com</div><div id=\"phone_number\">23344445555</div></div></body></html>", output_format: OutputFormat::Yaml};
        let result = extractor.abstract_objs_intermediate(&ids).unwrap();

        let mut expected_props_group1 = HashMap::new();
        expected_props_group1.insert("email".to_string(), PropertyValue::Str("abc@abc.com".to_string()));
        expected_props_group1.insert("phone-number".to_string(), PropertyValue::Str("13344445555".to_string()));
        let mut expected_props_group2 = HashMap::new();
        expected_props_group2.insert("email".to_string(), PropertyValue::Str("def@abc.com".to_string()));
        expected_props_group2.insert("phone-number".to_string(), PropertyValue::Str("23344445555".to_string()));
        let expected_output = vec![ IntermediateObject {object_id: "user-info".to_string(), records: vec![expected_props_group1, expected_props_group2]}];
        assert_eq!(result, expected_output);
    }

    #[test]
    fn abstrct_intermediate_multiple_objects() {
        let ids = construct_multiple_ident_obj();
        let extractor = Extractor { text: "<html><head></head><body><div id=\"user_info\"><div id=\"email\">abc@abc.com</div><div id=\"phone_number\">13344445555</div></div><div id=\"book_info\"><div id=\"isn\">123456</div><div id=\"price\">178.55</div></div></body></html>", output_format: OutputFormat::Yaml};
        let result = extractor.abstract_objs_intermediate(&ids).unwrap();

        let mut expected_props_for_obj1 = HashMap::new();
        expected_props_for_obj1.insert("email".to_string(), PropertyValue::Str("abc@abc.com".to_string()));
        let mut expected_props_for_obj2 = HashMap::new();
        expected_props_for_obj2.insert("isn".to_string(), PropertyValue::Str("123456".to_string()));
        let expected_output = vec![ IntermediateObject {object_id: "user-info".to_string(), records: vec![expected_props_for_obj1]},
                IntermediateObject {object_id: "book-info".to_string(), records: vec![expected_props_for_obj2]}];
        assert_eq!(result, expected_output);
    }

    #[test]
    fn abstract_value_from_element_property() {
        let ids = vec![ IdentObj {
            object_id: "user-info".to_owned(),
            css_selector: "div#user_info".to_owned(),
            properties: vec![Identity {
                    id: "link".to_owned(),
                    css_selector: "a".to_owned(),
                    value_type: IdentityValueType::Str,
                    value_from: ValueFrom::Property("href".to_string())
            }]
        }];
        let extractor = Extractor { text: "<html><head></head><body><div id=\"user_info\"><a href=\"mail_to:abc@abc.com\">abc@abc.com</div><div id=\"phone_number\">13344445555</div></div></body></html>", output_format: OutputFormat::Yaml};
        let result = extractor.abstract_objs(&ids).unwrap();
        assert_eq!(result, Output::Yaml(r#"---
- object_id: user-info
  records:
    - link: "mail_to:abc@abc.com"
"#.to_string()));
    }

    #[test]
    fn abstract_single_yaml() {
        let ids = vec![construct_ident_obj_single_prop()];
        let extractor = Extractor { text: "<html><head></head><body><div id=\"user_info\"><div id=\"email\">abc@abc.com</div><div id=\"phone_number\">13344445555</div></div></body></html>", output_format: OutputFormat::Yaml};
        let result = extractor.abstract_objs(&ids).unwrap();
        assert_eq!(result, Output::Yaml(r#"---
- object_id: user-info
  records:
    - email: abc@abc.com
"#.to_string()));
    }

    #[test]
    fn abstract_single_json() {
        let ids = vec![construct_ident_obj_single_prop()];
        let extractor = Extractor { text: "<html><head></head><body><div id=\"user_info\"><div id=\"email\">abc@abc.com</div><div id=\"phone_number\">13344445555</div></div></body></html>", output_format: OutputFormat::Json};
        let result = extractor.abstract_objs(&ids).unwrap();
        assert_eq!(result, Output::Json("[{\"object_id\":\"user-info\",\"records\":[{\"email\":\"abc@abc.com\"}]}]".to_string()));
    }
    #[test]
    fn abstract_multiple_records() {
        let ids = construct_multiple_ident_obj();
        let extractor = Extractor { text: "<html><head></head><body><div id=\"user_info\"><div id=\"email\">abc@abc.com</div><div id=\"phone_number\">13344445555</div></div><div id=\"book_info\"><div id=\"isn\">123456</div><div id=\"price\">178.55</div></div></body></html>", output_format: OutputFormat::Yaml};
        let result = extractor.abstract_objs(&ids).unwrap();
        assert_eq!(result, Output::Yaml(r#"---
- object_id: user-info
  records:
    - email: abc@abc.com
- object_id: book-info
  records:
    - isn: "123456"
"#.to_string()));
    }

    #[test]
    fn abstract_single_text() {
        let ids = vec![construct_ident_obj_single_prop()];
        let extractor = Extractor { text: "<html><head></head><body><div id=\"user_info\"><div id=\"email\">abc@abc.com</div><div id=\"phone_number\">13344445555</div></div></body></html>", output_format: OutputFormat::Text};
        let result = extractor.abstract_objs(&ids).unwrap();
        assert_eq!(result, Output::Text("abc@abc.com".to_string()));

    }

}
