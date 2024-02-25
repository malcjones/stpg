use maud::{html, Markup};

pub fn style() -> Markup {
    html! {
        style {
            (r#"
            body {
                font-family: sans-serif;
                margin: 0;
                padding: 0;
            }
            header {
                background-color: #f0f0f0;
                padding: 1em;
                text-align: center;
            }
            main {
                padding: 1em;
            }
            footer {
                background-color: #f0f0f0;
                padding: 1em;
                text-align: center;
            }
            .links {
                display: flex;
                flex-wrap: wrap;
                justify-content: center;
            }
            .category {
                margin: 1em;
                padding: 1em 2em 1em 2em;
                border: 1px solid #ccc;
                border-radius: 5px;
                box-shadow: 0 0 5px #ccc;
            }

            ul {
                list-style: none;
                padding: 0;
            }
            li {
                margin: 0.5em 0;
            }
            a {
                text-decoration: none;
                color: #000;
            }
            a:hover {
                text-decoration: underline;
            }
            "#)
        }
    }
}
