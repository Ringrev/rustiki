use moonlight::*;

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

    //---- Article -----
    GetArticles,
    AddArticle {
        title: String,
        content: String,
        author: String,
        tags: Vec<String>,
    },
    EditArticle {
        id: u32,
        new_title: String,
        new_content: String,
        new_contributors: Vec<String>,
        new_tags: Vec<String>,
    },
    RemoveArticle {
        id: u32,
    },
}

// ------ DownMsg ------

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum DownMsg {
    // ------ Auth ------
    LoggedIn(LocalUser),
    LoginError(String),
    RegistrationError(String),
    // ------Article-----
    Articles(Vec<LocalArticle>),
    ArticleAdded(String),
    ArticleUpdated,
    ArticleRemoved,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct LocalUser {
    pub id: String,
    pub email: String,
    pub username: String,
    pub auth_token: String,
}

impl LocalUser {
    pub fn new(id: String, email: String, username: String, auth_token: String) -> Self {
        Self {
            id,
            email,
            username,
            auth_token,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            id: "".to_string(),
            email: "".to_string(),
            username: "".to_string(),
            auth_token: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct LocalArticle {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub contributors: Vec<String>,
    pub author: String,
    pub tags: Vec<String>,
    pub created_time: String,
    pub updated_time: String,
}

impl LocalArticle {
    pub fn new(
        id: u32,
        title: String,
        content: String,
        contributors: Vec<String>,
        author: String,
        tags: Vec<String>,
        created_time: String,
        updated_time: String,
    ) -> Self {
        Self {
            id,
            title,
            content,
            contributors,
            author,
            tags,
            created_time,
            updated_time,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            id: 0,
            title: "".to_string(),
            content: "".to_string(),
            contributors: vec![],
            author: "".to_string(),
            tags: vec![],
            created_time: "".to_string(),
            updated_time: "".to_string(),
        }
    }
}
