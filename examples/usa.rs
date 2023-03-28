use wikipedia_api::*;

#[tokio::main]
async fn main() -> Result<(), WikiError>
{
    // Search for a page on wikipedia
    let page = Page::search("USA").await?;

    let title = &page.title.clone();

    // Get it's summary
    let page_summary = page.get_summary().await?;

    println!("{title} Summarized:\n{page_summary}");
    Ok(())
}
