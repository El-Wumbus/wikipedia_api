use wikipedia_api::*;

#[tokio::main]
async fn main() -> Result<(), WikiError> {
    // Search for a page on wikipedia
    let page = Page::search("USA").await?;

    // Get it's summary
    let page_summary = page.get_summary().await?;

    println!("{} Summarized:\n{page_summary}", page.get_title());
    Ok(())
}
