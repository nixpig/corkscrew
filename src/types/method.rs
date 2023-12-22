use std::ops::Index;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Method {}
impl Index<&'_ str> for Method {
    type Output = reqwest::Method;

    fn index(&self, index: &str) -> &reqwest::Method {
        match index {
            "get" => &reqwest::Method::GET,
            "post" => &reqwest::Method::POST,
            "put" => &reqwest::Method::PUT,
            "patch" => &reqwest::Method::PATCH,
            "delete" => &reqwest::Method::DELETE,
            _ => &reqwest::Method::GET,
        }
    }
}

#[cfg(test)]

mod test {
    use std::error::Error;

    use crate::Method;

    #[test]
    fn test_get_method_indexable() -> Result<(), Box<dyn Error>> {
        let got = Method {}["get"].clone();

        assert_eq!(got, &reqwest::Method::GET, "get should return GET method");

        Ok(())
    }

    #[test]
    fn test_post_method_indexable() -> Result<(), Box<dyn Error>> {
        let got = Method {}["post"].clone();

        assert_eq!(got, reqwest::Method::POST, "post should return POST method");

        Ok(())
    }

    #[test]
    fn test_put_method_indexable() -> Result<(), Box<dyn Error>> {
        let got = Method {}["put"].clone();

        assert_eq!(got, reqwest::Method::PUT, "put should return PUT method");

        Ok(())
    }

    #[test]
    fn test_patch_method_indexable() -> Result<(), Box<dyn Error>> {
        let got = Method {}["patch"].clone();

        assert_eq!(
            got,
            reqwest::Method::PATCH,
            "patch should return PATCH method"
        );

        Ok(())
    }

    #[test]
    fn test_delete_method_indexable() -> Result<(), Box<dyn Error>> {
        let got = Method {}["delete"].clone();

        assert_eq!(
            got,
            reqwest::Method::DELETE,
            "delete should return DELETE method"
        );

        Ok(())
    }

    #[test]
    fn test_default_method_fallback() -> Result<(), Box<dyn Error>> {
        let got = Method {}["unknown"].clone();

        assert_eq!(
            got,
            reqwest::Method::GET,
            "unknown should fallback and return GET method"
        );

        Ok(())
    }
}
