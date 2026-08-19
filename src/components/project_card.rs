use crate::models::Project;
use leptos::prelude::*;

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <article class="project-card">
        <div class="project-content">
        <h2>{project.title}</h2>
        <p>{project.description}</p>
        <ul class="project-tech">{project
                .tech
                .into_iter()
                .map(|tech| view! {
                    <li>{tech}</li>
                })
                .collect_view()}
        </ul>
        </div>

        {project.link.map(|link| view! {
        <a class="project-link" href=link target="_blank" rel="noopener noreferer">"View Project →"</a>

        })}
        </article>
    }
}
