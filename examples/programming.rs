use wikipedia_api::*;

fn main() -> Result<(), WikiError> {
    // Search for a page on wikipedia
    let page = Page::search("Programming Language")?;

    // Get it's summary
    let page_summary = page.get_summary()?;

    println!("Programming Language Summarized:\n{page_summary}");
    Ok(())
}
