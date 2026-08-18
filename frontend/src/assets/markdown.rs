use pulldown_cmark::{html, Options, Parser};
use yew::{AttrValue, Html};

pub fn markdown_to_html(markdown: &str) -> Html {
    let mut options = Options::empty();

    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);

    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);

    let safe_html = ammonia::clean(&html_output);

    Html::from_html_unchecked(
        AttrValue::from(safe_html)
    )
}