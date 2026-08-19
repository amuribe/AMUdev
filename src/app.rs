use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::components::project_card::ProjectCard;
use crate::models::project::Project;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                <link href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@300..700&family=Syne:wght@400..800&display=swap" rel="stylesheet" />
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
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // PROJECTS
    let amudev = Project {
        title: "AMUdev",
        description: "My personal portfolio website",
        tech: vec!["Rust", "Leptos", "Docker", "GitHub Actions"],
        link: Some("https://github.com/amuribe/AMUdev"),
    };

    let amuvie = Project {
        title: "AMUvie",
        description: "A custom movie tracking app and API",
        tech: vec!["PHP", "TypeScript", "React", "React-Router", "MySQL", "CSS"],
        link: Some("https://github.com/amuribe/AMUvie"),
    };

    view! {
        <main class="page-container">
            <section class="hero">
                <div class="hero-content">
                    <div class="hero-heading">
                        <p class="hero-eyebrow">"PORTFOLIO / 2026"</p>
                        <h1>"Alex McPherson Uribe"</h1>
                    </div>

                    <p class="hero-description">
                        "Computer Science student building software, systems, and creative technical projects."
                    </p>


                    <div class="hero-actions">
                        <a href="#projects" class="button button-primary">"View Projects"</a>
                        <a href="#contact" class="button button-secondary">"Contact"</a>
                    </div>
                </div>
            </section>

            <section id="projects" class="projects-section">
                <h2>"Selected Projects"</h2>

                <div class="project-grid">
                    <ProjectCard project=amudev/>
                    <ProjectCard project=amuvie/>
                </div>
            </section>
        </main>
    }
}
