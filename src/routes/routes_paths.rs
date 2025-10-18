use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug,EnumIter)]
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
