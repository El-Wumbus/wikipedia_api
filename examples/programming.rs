use wikipedia_api::*;

#[tokio::main]
async fn main() -> Result<(), WikiError> {
    // Search for a page on wikipedia
    let page = Page::search("Programming Language").await?;

    // Get it's summary
    let page_summary = page.get_summary().await?;

    println!("Programming Language Summarized:\n{page_summary}");
    Ok(())
}
