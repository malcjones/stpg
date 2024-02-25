use maud::{html, Markup, DOCTYPE};

use crate::site::style;

pub fn layout(title: &str, content: Markup, categories: Vec<super::Category>) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) }
                (style::style())
            }
            body {
                (header(title))
                main { (content) }
                (footer(categories.len(), categories.iter().map(|c| c.links.len()).sum()))
            }
        }
    }
}

fn header(title: &str) -> Markup {
    html! {
        header {
            h1 { (title) }
        }
    }
}

fn footer(cat_count: usize, link_count: usize) -> Markup {
    html! {
        footer {
            "this site uses " a href="https://maud.lambda.xyz" { "maud" } " | categories: " (cat_count) " | links: " (link_count)
        }
    }
}
