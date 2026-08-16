use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/amudev.css"/>

        // sets the document title
        <Title text="Welcome to AMUdev"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // STATES
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let count_on_click = move |_| *count.write() += 1;

    let show_contact = RwSignal::new(false);
    let show_contact_on_click = move |_| {
        show_contact.update(|value| {
            *value = !*value;
        })
    };

    // VARS
    let name = "AMUdev";
    let description = "View projects developed by Alex McPherson Uribe, find contact information, and view current work";

    view! {
        <h1>{ name }</h1>
        <ProjectIntro
            description=description
        />
        <button on:click=count_on_click>"Projects Viewed: " {count}</button>
        <button on:click=show_contact_on_click>{
            move || {
                if show_contact.get() {"Hide Contact"} else {"Show Contact"}
            }
        }</button>


        <Show when=move || show_contact.get()>
            <p>"Contact"</p>
        </Show>

    }
}
/// Renders an introduction section
#[component]
fn ProjectIntro(description: &'static str) -> impl IntoView {
    view! {
        <p>{description}</p>
    }
}
