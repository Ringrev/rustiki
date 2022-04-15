
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
    pub id: u32,
    pub title: String,
    pub content: String,
    pub contributors: Vec<User>,
    pub author: User,
    pub tags: Vec<Tag>,
    pub created_time: String,
    pub updated_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Tag {
    pub id: u32,
    pub text: String,
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
        author: User,
        tags: Vec<Tag>,
    },
    // org_title input needs to be replaced with ID when the Article object is expanded to include ID
    EditArticle {
        id: u32,
        new_title: String,
        new_content: String,
        new_contributors: Vec<User>,
        new_tags: Vec<Tag>,
    },
    // This input needs to be replaced with ID when the Article object is expanded to include ID
    RemoveArticle {
        id: u32,
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