use maud::{html, Markup};

struct Route {
    text: &'static str,
    path: &'static str,
}

const ROUTES: [Route; 2] = [
    Route {
        text: "Emails",
        path: "/emails",
    },
    Route {
        text: "Events",
        path: "/events",
    },
];

fn get_link(url: &str, route: &Route) -> Markup {
    println!("fuck {}", { url });
    let mut class_list = String::from("text-lg mx-4");
    if url.starts_with(route.path) {
        class_list += " bg-red-50"
    }
    html! {
       a class=(class_list) href=(route.path) { (route.text) }
    }
}

pub fn build(content: Markup, url: &str) -> Markup {
    html! {
        html {
            head {
                link rel="stylesheet" href="/assets/main.css" {}
            }
            body class="overflow-hidden flex flex-col h-screen" {
                script src="/assets/main.js" {}
                nav class="border-b-1 border-stone-100 p-4 w-full" {
                    @for route in &ROUTES {
                        (get_link(url, route))
                        // a class="text-lg mr-4" href="/emails" { "Emails" }
                    }
                    // a class="text-lg mr-4" href="/emails" { "Emails" }
                    // a class="text-lg" href="/events" { "All Events" }
                }
                div class="flex flex-col grow min-h-0" {
                    (content)
                }
            }
        }
    }
}
