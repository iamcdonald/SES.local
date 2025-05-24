use maud::{html, Markup};

pub fn build(content: Markup) -> Markup {
    html! {
        html {
            head {
                link rel="stylesheet" href="/assets/main.css" {}
            }
            body class="overflow-hidden" {
                script src="/assets/main.js" {}
                (content)
            }
        }
    }
}
