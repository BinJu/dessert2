/// The output represents the text after the abstraction from the source.
/// It could be either Json format, or Yaml format, or plain text. Or Nothing if nothing could be
/// extracted from the source.
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Output {
    Json(String),
    Yaml(String),
    Text(String),
    Empty
}

impl<T: Into<String> + Sized> std::convert::From<T> for Output {
    fn from(item: T) -> Self {
        let text = item.into().trim().to_owned();
        if text.is_empty() {
            Output::Empty
        } else {
            Output::Text(text)
        }
    }
}

impl Output {
    #[allow(dead_code)]
    #[inline]
    pub fn from_json<T: Into<String>>(json: T) -> Self {
        Self::Json(json.into())
    }
    #[allow(dead_code)]
    #[inline]
    pub fn from_yaml<T: Into<String>>(yaml: T) -> Self {
        Self::Yaml(yaml.into())
    }
    #[allow(dead_code)]
    #[inline]
    pub fn json<C: Into<String>>(val: C) -> Self {
        Self::Json(val.into()) 
    }

    #[allow(dead_code)]
    #[inline]
    pub fn yaml<C: Into<String>>(val: C) -> Self {
        Self::Yaml(val.into())
    }

    #[allow(dead_code)]
    #[inline]
    pub fn text<C: Into<String>>(val: C) -> Self {
        Self::Text(val.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_json() {
        let output = Output::from_json("{}");
        assert_eq!(output, Output::json("{}"));
    }

    #[test]
    fn from_yaml() {
        let output = Output::from_yaml("---");
        assert_eq!(output, Output::yaml("---"));
    }

    #[test]
    fn from_plain_text() {
        let output = Output::from("this is a dummy text");
        assert_eq!(output, Output::text("this is a dummy text"));
    }

    #[test]
    fn from_emty() {
        let output = Output::from("");
        assert_eq!(output, Output::Empty);
    }

    #[test]
    fn from_emty_with_spaces() {
        let output = Output::from(" ");
        assert_eq!(output, Output::Empty);
    }
}

