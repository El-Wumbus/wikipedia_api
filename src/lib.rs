//! Wikipedia api crate

#[derive(Defualt, Clone, PartialEq, PartialOrd, Ord, Eq, Debug)]
pub enum WikiError
{
    /// The searched page wasn't found. The search term is stored in `String`
    PageNotFoundError(String),

    /// Making a wikipedia request failed
    PageRequestError,

    /// Error parsing the returned JSON
    JsonParseError,
}

impl std::fmt::Display for WikiError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = match self
        {
            Self::PageNotFoundError(e) => format!("PageNotFound: Couldn't find '{e}'."),
            Self::PageRequestError => {
                let m = "PageRequestError: Internal error.";
                error!("{m}");
                m
            },
            Self::JsonParseError => {
                let m = "JsonParseError: Internal response parsing error."
                error!("{m}");
                m
            }
        };
        
        write!(f, "{m}")
    }
}


#[derive(Defualt, Clone, PartialEq, PartialOrd, Ord, Eq, Debug)]
/// The result of a search operation.
pub struct Page
{
    /// Title of the page
    title: String,

    /// The URL of the page
    url: String,
}

#[derive(Defualt, Clone, PartialEq, PartialOrd, Ord, Eq, Debug)]
pub enum PageContent
{
    All(String),
    Summary(String),
}

impl std::fmt::Display for PageContent
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = match self
        {
            Self::Summary(x) => x,
            Self::All(x) => x,
        };

        write!(f, "{m}")
    }
}

impl Page
{
    /// Create a new `Page`
    pub fn new(title: String, url: String) -> Self
    {
        Self {title, url}
    }

    /// Search for a page on Wikipedia and return a `Page`
    pub async fn search(title: String) -> Result<Self, WikiError>
    {
        type search_result =  (String, Vec<String>, Vec<String>, Vec<String>);
        // Replace spaces with %20 for the url
        title.replace(" ", "%20");
        let request_url =
        format!(
            "https://en.wikipedia.org/w/api.php\
            ?action=opensearch&search={}&limit=1&namespace=0&format=json",
            title.trim()
        );

        /// Make the API call, parse the json to a SearchResult.
        if let Ok(resp:search_result) = {
            match reqwest::get(&request_url).await
            {
                Ok(x) => x,
                Err(_) => return Err(WikiError::PageRequestError),
            }
            .json()
            .await
        }
        {
            Ok(Self::new(resp.2.get(0)?, resp.4.get(0)?))
        }
        else
        {
            Err(WikiError::JsonParseError);
        }

    }

    pub async get_summary(&self) -> String
    {
        let request_url =
        format!(
            "https://en.wikipedia.org/w/api.php?action=query&prop=extracts&titles={}\
            &exintro&exlimit=1&explaintext=1&exsectionformat=plain&format=json",
            self.title
        )

        /// Make the API call, parse the json to a SearchResult.
        if let Ok(resp:search_result) = {
            match reqwest::get(&request_url).await
            {
                Ok(x) => x,
                Err(_) => return Err(WikiError::PageRequestError),
            }
            .json()
            .await
        }
        {
            Ok(Self::new(resp.2.get(0)?, resp.4.get(0)?))
        }
        else
        {
            Err(WikiError::JsonParseError);
        }
    }


}


pub struct 