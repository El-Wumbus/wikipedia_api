use std::rc::Rc;

use wikipedia_api::*;

fn main() -> Result<(), WikiError> {
    // Search for a page on wikipedia
    let page = Page::search("USA")?;

    let title = Rc::clone(&page.title);

    // Get it's summary
    let page_summary = page.get_summary()?;

    println!("{title} Summarized:\n{page_summary}");
    Ok(())
}
