use maud::{html, Markup};

pub fn style() -> Markup {
    html! {
        style {
            (include_str!("../static/app.css"))
        }
    }
}
