use std::sync::Arc;
use moonlight::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    pub auth_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Article {
    //pub id: String,
    pub title: String,
    pub content: String,
    //pub contributor: String,
    //pub tags: String,
}

// ------ UpMsg ------

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum UpMsg {

    // ------ Auth ------
    Login {
        email: String,
        password: String,
    },
    Register {
        email: String,
        password: String,
        username: String,
    },
    // Logout,

    //---- Article -----
    GetArticles,
    AddArticle {
        title: String,
        content: String,
    },
    // org_title input needs to be replaced with ID when the Article object is expanded to include ID
    EditArticle {
        org_title: String,
        new_title: String,
        new_content: String,
    },
    // This input needs to be replaced with ID when the Article object is expanded to include ID
    RemoveArticle {
        title: String,
    },
}

// ------ DownMsg ------

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum DownMsg {

    // ------ Auth ------
    LoginError(String),
    RegistrationError(String),
    LoggedIn(User),
    // LoggedOut,
    // ------Article-----
    Articles(Vec<Article>),
    ArticleAdded(String),
    ArticleUpdated,
    ArticleRemoved,
}