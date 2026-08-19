pub struct Project {
    pub title: &'static str,
    pub description: &'static str,
    pub tech: Vec<&'static str>,
    // Optional link if project has one
    pub link: Option<&'static str>,
}
