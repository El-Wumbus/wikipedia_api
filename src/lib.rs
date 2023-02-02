//! Wikipedia api crate

use log::{error, info};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Debug)]
pub enum WikiError<'a> {
    /// The searched page wasn't found. The search term is stored in `String`
    PageNotFoundError(&'a str),

    /// Making a wikipedia request failed
    PageRequestError,

    /// Error parsing the JSON
    JsonParseError,

    /// An error with the Wikipedia api response
    ResponseError,
}

impl std::fmt::Display for WikiError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = match self {
            Self::PageNotFoundError(e) => format!("PageNotFound: Couldn't find '{e}'."),
            Self::PageRequestError => {
                let m = "PageRequestError: Internal error.";
                error!("{m}");
                m.to_string()
            }
            Self::JsonParseError => {
                let m = "JsonParseError: Internal response parsing error.";
                error!("{m}");
                m.to_string()
            }
            Self::ResponseError => {
                let m = "ResponseError: Wikipedia returned an unexpected result.";
                error!("{m}");
                m.to_string()
            }
        };

        write!(f, "{m}")
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RPage {
    pub pageid: i64,
    pub ns: i64,
    pub title: String,
    pub extract: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub pages: Vec<RPage>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryResponse {
    pub batchcomplete: bool,
    pub query: Query,
}

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Debug)]
/// The result of a search operation.
pub struct Page {
    /// Title of the page
    title: Rc<str>,

    /// The URL of the page
    url: Rc<str>,
}

impl Page {
    /// Create a new `Page`
    pub fn new(title: String, url: String) -> Self {
        Self { title: Rc::from(title), url: Rc::from(url) }
    }

    pub fn get_title(&self) -> Rc<str>
    {
        self.title.clone()
    }

    pub fn get_url(&self) -> Rc<str>
    {
        self.url.clone()
    }

    /// Search for a page on Wikipedia and return a `Page`
    pub async fn search(search_term: &str) -> Result<Self, WikiError> {
        type SearchResult = (String, Vec<String>, Vec<String>, Vec<String>);

        // Replace spaces with %20 for the url
        let title = search_term.replace(' ', "%20");

        let request_url =
        format!(
            "https://en.wikipedia.org/w/api.php?action=opensearch&search={}&limit=1&namespace=0&format=json",
            title.trim()
        );
        let page;

        // Make the API call, parse the json to a `Page`.
        if let Ok(resp) = {
            match reqwest::get(&request_url).await {
                Ok(x) => {
                    info!("Requested '{}'", request_url);
                    x
                }
                Err(_) => return Err(WikiError::PageRequestError),
            }
            .json::<SearchResult>()
            .await
        } {
            let t = match resp.1.get(0) {
                Some(x) => x.to_string(),
                None => return Err(WikiError::PageNotFoundError(search_term)),
            };

            let u = match resp.3.get(0) {
                Some(x) => x.to_string(),
                None => return Err(WikiError::PageNotFoundError(search_term)),
            };

            page = Self::new(t, u);
        } else {
            return Err(WikiError::JsonParseError);
        }
        Ok(page)
    }

    pub async fn get_summary(&self) -> Result<String, WikiError> {
        let request_url =
        format!(
            "https://en.wikipedia.org/w/api.php?action=query&format=json&prop=extracts&titles={}&formatversion=2&exchars=1000&explaintext=1&redirects=1",
            self.title
        );

        // Make the API call, parse the json to a `Page`.
        let resp = match {
            match reqwest::get(&request_url).await {
                Ok(x) => {
                    info!("Requested '{}'", request_url);
                    x
                }
                Err(_) => return Err(WikiError::PageRequestError),
            }
            .json::<SummaryResponse>()
            .await
        } {
            Ok(x) => x,
            Err(_) => return Err(WikiError::JsonParseError),
        };

        let summary_text = match resp.query.pages.get(0) {
            Some(x) => x,
            None => return Err(WikiError::ResponseError),
        }
        .extract
        .to_owned();
        
        Ok(summary_text)
    }
}

#[cfg(test)]

pub mod tests {
    use super::{Page, WikiError};

    #[tokio::test]
    async fn test_search_page() {
        let expected_page = Page::new(
            "Albert Einstein".to_string(),
            "https://en.wikipedia.org/wiki/Albert_Einstein".to_string(),
        );
        let page = Page::search("Albert Einstein").await.unwrap();
        assert_eq!(page, expected_page);
    }

    #[tokio::test]
    async fn test_search_page_misspelled() {
        let expected_page = Page::new(
            "Programming language".to_string(),
            "https://en.wikipedia.org/wiki/Programming_language".to_string(),
        );
        let page = Page::search("progrmming lang").await.unwrap();
        assert_eq!(page, expected_page);
    }

    #[tokio::test]
    async fn test_search_page_not_found() {
        let page = Page::search("this page does not exist")
            .await
            .err()
            .unwrap();
        assert_eq!(
            page,
            WikiError::PageNotFoundError("this page does not exist".to_string())
        );
    }

    #[tokio::test]
    async fn test_get_page_summary() {
        let page = Page::search("Albert Einstein").await.unwrap();
        let r = page.get_summary().await;
        assert!(r.is_ok());
    }
}
