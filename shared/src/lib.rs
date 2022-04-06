use moonlight::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct User {
    pub id: String,
    pub email: String,
    pub auth_token: String,
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
    // Logout,
    // // ------ Organization ------
    // GetOrganizations,
    // AddOrganization(OrganizationId, String),
    // RemoveOrganization(OrganizationId),
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
    // LoginError(String),
    LoggedIn(User),
    // LoggedOut,
    // AuthorizationError(String),
    // // ------ Organization ------
    // Organizations(Vec<Organization>),
    // OrganizationAdded,
    // OrganizationRemoved,
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