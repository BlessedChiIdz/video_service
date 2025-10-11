//#[derive(Debug, Clone)]
pub enum AppRoute {
    Main,
    Video
}

 impl AppRoute {
    pub fn path(&self) -> &'static str {
        match self {
            AppRoute::Main => "/",
            AppRoute::Video => "/video/{name}"
        }
    }
}
