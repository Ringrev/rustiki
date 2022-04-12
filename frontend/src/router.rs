use std::collections::VecDeque;
use zoon::{println,*};
use crate::{app::{self, PageName}};

// ------ Route history ------

#[static_ref]
fn route_history() -> &'static Mutable<VecDeque<Route>> {
    Mutable::new(VecDeque::new())
}

fn push_to_route_history(route: Route) {
    let mut history = route_history().lock_mut();
    if history.len() == 2 {
        history.pop_back();
    }
    history.push_front(route);
}

pub fn previous_route() -> Option<Route> {
    route_history().lock_ref().get(1).cloned()
}

// ------ router ------

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route: Option<Route>| {

        let route = match route {
            Some(route) => {
                push_to_route_history(route.clone());
                route
            }
            None => {
                return app::set_page_name(PageName::Unknown);
            }
        };

        match route {
            Route::Root => {
                app::set_page_name(PageName::Home);
            }
            Route::Registration => {
                app::set_page_name(PageName::Registration);
            }
            Route::NewArticle => {
                app::set_page_name(PageName::NewArticle);
            }
            Route::LogIn => {
                app::set_page_name(PageName::LogIn);
            }
            Route::ViewArticle => {
                app::set_page_name(PageName::ViewArticle);
            }
        }
    })
}

// ------ Route ------

#[route]
#[derive(Clone)]
pub enum Route {
    #[route("registration")]
    Registration,

    #[route("new_article")]
    NewArticle,

    #[route("view_article")]
    ViewArticle,

    #[route("log_in")]
    LogIn,

    #[route()]
    Root,
}