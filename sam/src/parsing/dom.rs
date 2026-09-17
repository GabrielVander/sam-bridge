use scraper::ElementRef;

pub fn find_descendant<'a>(root: ElementRef<'a>, tag: &str) -> Option<ElementRef<'a>> {
    root.descendent_elements()
        .find(|el| el.value().name() == tag)
}

pub fn find_descendant_with_id<'a>(
    root: ElementRef<'a>,
    tag: &str,
    id: &str,
) -> Option<ElementRef<'a>> {
    root.descendent_elements()
        .find(|el| el.value().name() == tag && el.value().id() == Some(id))
}

pub fn descendants_with_tag<'a>(root: ElementRef<'a>, tag: &str) -> Vec<ElementRef<'a>> {
    root.descendent_elements()
        .filter(|el| el.value().name() == tag)
        .collect()
}

pub fn text_content(element: ElementRef<'_>) -> String {
    element
        .text()
        .collect::<Vec<&str>>()
        .join(" ")
        .trim()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::{descendants_with_tag, find_descendant, find_descendant_with_id, text_content};
    use scraper::Html;

    #[test]
    fn find_descendant_locates_a_nested_tag() {
        let html = Html::parse_fragment("<div><section><span>hi</span></section></div>");
        let root = html.root_element();

        let found = find_descendant(root, "span").expect("span should be found");

        assert_eq!(found.value().name(), "span");
    }

    #[test]
    fn find_descendant_returns_none_when_the_tag_is_absent() {
        let html = Html::parse_fragment("<div><section></section></div>");
        let root = html.root_element();

        assert!(find_descendant(root, "span").is_none());
    }

    #[test]
    fn find_descendant_returns_the_first_match_in_document_order() {
        let html = Html::parse_fragment("<div><p id=\"first\"></p><p id=\"second\"></p></div>");
        let root = html.root_element();

        let found = find_descendant(root, "p").expect("a paragraph should be found");

        assert_eq!(found.value().id(), Some("first"));
    }

    #[test]
    fn find_descendant_with_id_matches_both_tag_and_id() {
        let html = Html::parse_fragment("<div id=\"wrong\"></div><table id=\"target\"></table>");
        let root = html.root_element();

        let found = find_descendant_with_id(root, "table", "target").expect("table should match");

        assert_eq!(found.value().name(), "table");
        assert_eq!(found.value().id(), Some("target"));
    }

    #[test]
    fn find_descendant_with_id_ignores_a_matching_tag_with_the_wrong_id() {
        let html = Html::parse_fragment("<table id=\"other\"></table>");
        let root = html.root_element();

        assert!(find_descendant_with_id(root, "table", "target").is_none());
    }

    #[test]
    fn find_descendant_with_id_ignores_a_matching_id_on_the_wrong_tag() {
        let html = Html::parse_fragment("<div id=\"target\"></div>");
        let root = html.root_element();

        assert!(find_descendant_with_id(root, "table", "target").is_none());
    }

    #[test]
    fn descendants_with_tag_collects_every_match_in_document_order_regardless_of_depth() {
        let html = Html::parse_fragment(
            "<table><tbody><tr id=\"1\"></tr><tr id=\"2\"><td><table><tr id=\"nested\"></tr></table></td></tr></tbody></table>",
        );
        let root = html.root_element();

        let rows = descendants_with_tag(root, "tr");

        let ids: Vec<Option<&str>> = rows.iter().map(|row| row.value().id()).collect();
        assert_eq!(ids, vec![Some("1"), Some("2"), Some("nested")]);
    }

    #[test]
    fn descendants_with_tag_returns_empty_when_nothing_matches() {
        let html = Html::parse_fragment("<div></div>");
        let root = html.root_element();

        assert_eq!(descendants_with_tag(root, "tr").len(), 0);
    }

    #[test]
    fn text_content_joins_multiple_text_nodes_with_a_space_and_trims() {
        let html = Html::parse_fragment(
            "<table><tr><td>  Revisão: <b>Ligaduras</b>. Estudar.  </td></tr></table>",
        );
        let cell = find_descendant(html.root_element(), "td").expect("td should be found");

        assert_eq!(text_content(cell), "Revisão:  Ligaduras . Estudar.");
    }

    #[test]
    fn text_content_of_an_empty_element_is_empty() {
        let html = Html::parse_fragment("<table><tr><td></td></tr></table>");
        let cell = find_descendant(html.root_element(), "td").expect("td should be found");

        assert_eq!(text_content(cell), "");
    }
}
