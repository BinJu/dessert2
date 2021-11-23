use extract::OutputFormat;
use idents::IdentObj;
use std::env;

mod idents;
mod output;
mod extract;
mod params;

/// Main entry. Those parameters are acceptable:
/// `--url String`: Read html text from the given url. If this parameter is absent, the propgram
/// will read the HTML from stdin.
/// `--template String`: The template that the propgram uses to extract content from the HTML. The
/// program treats it as yaml if the text start with `---`, or else it is `json`. The
/// example of the template format:
/// ```
/// ---
/// object_id: detail-info
/// css_selector: "id=\"abc\""
/// properties:
///   - id: name
///     css_selector: "id=\"name\""
///     value_type: Str
///     value_from: InnerText
///   - id: address
///     css_selector: "id=\"address\""
///     value_type: Str
///     value_from: InnerText
/// ```
/// `--template-file String`: The template file that the program uses to extract content form the
/// HTML.
/// `--output-format String`: This could either be `json`, `yaml`, or `text`. The default value is
/// `yaml`. If `text` is given, it will print out only the first property value from the result.
fn main() {
    let params = params::parse_params(env::args());
    let html = params::read_html(&params);
    let template = params::read_template(&params);
    let output_format = params::parse_output_format(&params);

    let output = parse(&*html, &template, output_format);
    println!("{}", output);
}

#[allow(dead_code)]
fn parse(src: &str, abstract_template: &Vec<IdentObj>, output_format: OutputFormat) -> String {
    let extractor = extract::Extractor::new(src, output_format);
    match extractor.abstract_objs(abstract_template).unwrap() {
        output::Output::Json(text) => text,
        output::Output::Yaml(text) => text,
        output::Output::Text(text) => text,
        _ => "".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use idents::{Identity, IdentityValueType, ValueFrom};

    #[test]
    fn test_parse() {
        let source_html = r#"<html><head></head><body><div id="user_info"><div id="address">Ontario, Canada</div><div id="email">abc@abc.com</div></div></body></html>"#;
        let text = parse(source_html, &vec![ IdentObj {
            object_id: "user-info".to_owned(),
            css_selector: "div#user_info".to_owned(),
            properties: vec![Identity {
                    id: "address".to_owned(),
                    css_selector: "div#address".to_owned(),
                    value_type: IdentityValueType::Str,
                    value_from: ValueFrom::InnerText
            }]
        }], OutputFormat::Yaml);

        assert_eq!(text, r#"---
- object_id: user-info
  records:
    - address: "Ontario, Canada"
"#)
    }
}
