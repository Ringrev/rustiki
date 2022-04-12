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

    // // ------ Auth ------
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
    // // ------ Organization ------
    // GetOrganizations,
    // AddOrganization(OrganizationId, String),
    // RemoveOrganization(OrganizationId),
    //---- Article -----
    //GetArticles,
    AddArticle {
        title: String,
        content: String,
    },
    //RemoveArticle(ArticleId),



    // // ------ Mail ------
    // SendMail {
    //     from: String,
    //     to: String,
    //     subject: String,
    //     content: String,
    // }
}

// ------ DownMsg ------

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum DownMsg {

    // // ------ Auth ------
    LoginError(String),
    RegistrationError(String),
    LoggedIn(User),
    // LoggedOut,
    // AuthorizationError(String),
    // // ------ Organization ------
    // Organizations(Vec<Organization>),
    // OrganizationAdded,
    // OrganizationRemoved,
    //------Article-----
   // Articles(Vec<Articles>),
    ArticleAdded(String),
   // ArticleRemoved,
    // // ------ Mail ------
    // MailSent,
    // // ------ Other ------
    // ServerError(String),
}

// ------ Transfer objects ------

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(crate = "serde")]
// pub struct Article {
//     pub id: String,
//     pub name: String,
// }