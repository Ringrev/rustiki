//! Handles the routing of the webpage.
use crate::app::{self, PageName};
use std::collections::VecDeque;
use zoon::*;

// ------ ------
//    Commands
// ------ ------

/// Holds current and previous route.
/// When a new route is added, it is pushed to the front of the queue.
/// If there are 2 routes already, the element in the back of the queue is removed.
///
/// # Arguments
/// * `route` - The Route to add to route_history.
fn push_to_route_history(route: Route) {
    let mut history = route_history().lock_mut();
    if history.len() == 2 {
        history.pop_back();
    }
    history.push_front(route);
}

// ------ ------
//    States
// ------ ------

/// State of route history as a double-ended queue.
#[static_ref]
pub fn route_history() -> &'static Mutable<VecDeque<Route>> {
    Mutable::new(VecDeque::new())
}

/// State of the current route.
/// Defines what to set page to on each change in route.
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
            Route::Home => {
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
            Route::ViewArticle { article_id } => {
                app::set_page_name(PageName::ViewArticle);
            }
            Route::EditArticle => app::set_page_name(PageName::EditArticle),
        }
    })
}

// ------ ------
//    Types
// ------ ------

/// Represents the path of a route.
#[route]
#[derive(Clone)]
pub enum Route {
    #[route("registration")]
    Registration,

    #[route("new_article")]
    NewArticle,

    #[route("article", article_id)]
    ViewArticle { article_id: String },

    #[route("login")]
    LogIn,

    #[route()]
    Home,

    #[route("edit_article")]
    EditArticle,
}
