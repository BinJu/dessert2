use serde::{Serialize, Deserialize};

pub trait Concentrating {

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    pub id: String,
    pub css_selector: String,
    pub value_type: IdentityValueType,
    pub value_from: ValueFrom
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum IdentityValueType {
    Int,
    Float,
    Str,
    Bool
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ValueFrom {
    InnerText,
    Property(String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentObj {
    pub object_id: String,
    pub css_selector: String,
    pub properties: Vec<Identity>,
}

pub fn from_json_str(json_str: &str) -> Result<Vec<IdentObj>, serde_json::Error> {
    serde_json::from_str(json_str)
}

pub fn from_yaml_str(yaml_str: &str) -> Result<Vec<IdentObj>, serde_yaml::Error> {
    serde_yaml::from_str(yaml_str)
}

#[allow(dead_code)]
pub fn to_json_str(idents: &Vec<IdentObj>) -> Result<String, serde_json::Error> {
    serde_json::to_string(idents)
}

#[allow(dead_code)]
pub fn to_yaml_str(idents: &Vec<IdentObj>) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(idents)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[inline]
    fn construct_ident_obj() -> Vec<IdentObj> {
        vec![IdentObj {
                object_id: "detail-info".to_owned(),
                css_selector: "div#detail_info".to_owned(),
                properties: vec![Identity {
                        id: "email".to_owned(),
                        css_selector: "div#email".to_owned(),
                        value_type: IdentityValueType::Str,
                        value_from: ValueFrom::InnerText
                }, Identity {
                        id: "address".to_owned(),
                        css_selector: "div#address".to_owned(),
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
                    }, Identity {
                            id: "price".to_owned(),
                            css_selector: "div#price".to_owned(),
                            value_type: IdentityValueType::Str,
                            value_from: ValueFrom::InnerText
            }]
        }]
    }

    #[test]
    fn serialize_to_yaml() {
        let idents = construct_ident_obj();
        let s = to_yaml_str(&idents).unwrap();
        assert_eq!(s, r#"---
- object_id: detail-info
  css_selector: "div#detail_info"
  properties:
    - id: email
      css_selector: "div#email"
      value_type: Str
      value_from: InnerText
    - id: address
      css_selector: "div#address"
      value_type: Str
      value_from: InnerText
- object_id: book-info
  css_selector: "div#book_info"
  properties:
    - id: isn
      css_selector: "div#isn"
      value_type: Str
      value_from: InnerText
    - id: price
      css_selector: "div#price"
      value_type: Str
      value_from: InnerText
"#);
    }

    #[test]
    fn serialize_to_json() {
        let ident = construct_ident_obj();
        let s = serde_json::to_string(&ident).unwrap();
        assert_eq!(s, r#"[{"object_id":"detail-info","css_selector":"div#detail_info","properties":[{"id":"email","css_selector":"div#email","value_type":"Str","value_from":"InnerText"},{"id":"address","css_selector":"div#address","value_type":"Str","value_from":"InnerText"}]},{"object_id":"book-info","css_selector":"div#book_info","properties":[{"id":"isn","css_selector":"div#isn","value_type":"Str","value_from":"InnerText"},{"id":"price","css_selector":"div#price","value_type":"Str","value_from":"InnerText"}]}]"#);
    }

    #[test]
    fn deserialize_from_yaml() {
        let idents = construct_ident_obj();
        let s = serde_yaml::to_string(&idents).unwrap();
        let deserialized: Vec<IdentObj> = serde_yaml::from_str(&s).unwrap();
        assert_eq!(idents, deserialized);
    }

    #[test]
    fn deserialize_from_json() {
        let ident = construct_ident_obj();
        let s = serde_json::to_string(&ident).unwrap();
        let deserialized: Vec<IdentObj> = serde_json::from_str(&s).unwrap();

        assert_eq!(ident, deserialized);
    }
}
