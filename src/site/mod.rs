use maud::{html, Markup};

pub mod layout;
pub mod spf;
pub mod style;
#[derive(Debug, Clone)]
pub struct Link {
    pub url: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Category {
    pub name: String,
    pub links: Vec<Link>,
}

pub fn start_page(categories: Vec<Category>) -> Markup {
    let content = html! {
        div class="links" {
            @for category in categories.clone() {
                div class="category" {
                    h2 { (category_icon(&category.name)) span class="small" { (category.name) } }
                    ul {
                        @for link in category.links {
                            li {
                                // Adding a title attribute for the tooltip
                                a href=(link.url) title=(link.description) { (link.title) }
                            }
                        }
                    }
                }
            }
        }
    };
    layout::layout("stpg", content, categories)
}


pub fn category_icon(category: &str) -> &'static str {
    match category {
        "Developer" => "👨‍💻",
        "Important" => "🌟",
        "Finance" => "💰",
        "Programming" => "⚙️",
        "Torrent" => "🌊",
        _ => "❓",
    }
}
