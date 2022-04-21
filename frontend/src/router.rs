use std::collections::VecDeque;
use zoon::*;
use crate::{app::{self, PageName}};

// ------ ------
//    Commands
// ------ ------

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

#[static_ref]
fn route_history() -> &'static Mutable<VecDeque<Route>> {
    Mutable::new(VecDeque::new())
}

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
            // Route::ViewArticle {
            //     article_id,
            // } => {
            //     // view_article_page::set_expression();
            //     app::set_page_name(PageName::ViewArticle);
            // }
            Route::ViewArticle  => {
                app::set_page_name(PageName::ViewArticle);
            }
            Route::EditArticle => {
                app::set_page_name(PageName::EditArticle)
            }
        }
    })
}

// ------ ------
//    Types
// ------ ------

#[route]
#[derive(Clone)]
pub enum Route {
    #[route("registration")]
    Registration,

    #[route("new_article")]
    NewArticle,

    // #[route("article", article_id)]
    // ViewArticle {
    //     article_id: String,
    // },
    #[route("view_article")]
    ViewArticle,

    #[route("login")]
    LogIn,

    #[route()]
    Home,

    #[route("edit_article")]
    EditArticle,
}
