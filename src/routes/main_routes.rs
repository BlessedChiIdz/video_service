use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum AppRoute {
    Main,
    VideoGet,
    VideoUpload,
}

impl AppRoute {
    pub fn path(&self) -> &'static str {
        match self {
            AppRoute::Main => "/",
            AppRoute::VideoGet => "/video/{*key}",
            AppRoute::VideoUpload => "/video/upload",
        }
    }

    pub fn method(&self) -> &'static str {
        match self {
            AppRoute::Main => "GET",
            AppRoute::VideoGet => "GET",
            AppRoute::VideoUpload => "POST",
        }
    }

    pub fn generic_iterator<E, F>(pred: F)
    where
        E: IntoEnumIterator,
        F: Fn(E),
    {
        for e in E::iter() {
            pred(e)
        }
    }
}