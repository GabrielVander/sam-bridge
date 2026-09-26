pub fn find_descendant<'a>(
    root: scraper::ElementRef<'a>,
    tag: &str,
) -> Option<scraper::ElementRef<'a>> {
    root.descendent_elements()
        .find(|el| el.value().name() == tag)
}

pub fn find_descendant_with_id<'a>(
    root: scraper::ElementRef<'a>,
    tag: &str,
    id: &str,
) -> Option<scraper::ElementRef<'a>> {
    root.descendent_elements()
        .find(|el| el.value().name() == tag && el.value().id() == Some(id))
}

pub fn descendants_with_tag<'a>(
    root: scraper::ElementRef<'a>,
    tag: &str,
) -> Vec<scraper::ElementRef<'a>> {
    root.descendent_elements()
        .filter(|el| el.value().name() == tag)
        .collect()
}

pub fn text_content(element: scraper::ElementRef<'_>) -> String {
    element
        .text()
        .collect::<Vec<&str>>()
        .join(" ")
        .trim()
        .to_owned()
}
