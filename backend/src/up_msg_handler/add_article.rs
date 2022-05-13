//! Defines functions used for adding an article to database.
use crate::models::Article;
use aragog::query::{Comparison, Filter};
use aragog::{DatabaseConnection, DatabaseRecord, Record};
use rand::Rng;
use shared::DownMsg;

/// The handler for adding article to DB. Returns a DownMsg containing an empty message.
///
/// # Arguments
/// * `title` - A String holding the title of the article.
/// * `content` - A String holding the content of the article.
/// * `author` - A String holding the author of the article.
/// * `tags` - A vector of Strings holding the article's tags.
pub async fn handler(
    title: String,
    content: String,
    author: String,
    tags: Vec<String>,
    db_conn: &DatabaseConnection,
) -> DownMsg {
    create_article_in_db(title, content, author, tags, db_conn).await;
    // TODO: ArticleAdded enum might as well not return any arguments.
    DownMsg::ArticleAdded("".to_string())
}

/// Creates article in ArangoDB database.
///
/// # Arguments
/// * `title` - A String holding the title of the article.
/// * `content` - A String holding the content of the article.
/// * `author` - A String holding the author of the article.
/// * `tags` - A vector of Strings holding the article's tags.
pub async fn create_article_in_db(
    title: String,
    content: String,
    author: String,
    tags: Vec<String>,
    db_conn: &DatabaseConnection,
) {
    let db_article = Article::new(
        generate_id(db_conn).await,
        title,
        content,
        vec![],
        author,
        tags,
        super::get_time(),
        super::get_time(),
    );
    DatabaseRecord::create(db_article, db_conn).await.unwrap();
}

/// Returns an id as u32 when the generated id is unique.
async fn generate_id(db_conn: &DatabaseConnection) -> u32 {
    let mut rand = rand::thread_rng();
    let mut id: u32 = 0;
    let mut checking = true;
    while checking {
        id = rand.gen::<u32>();
        if check_id_unique(id.clone(), db_conn).await {
            checking = false;
        }
    }
    id.clone()
}

/// Returns <code>true</code> if id is unique.
/// Returns <code>false</code> if id is not unique.
///
/// # Arguments
/// * `id` - A u32 value holding an id.
async fn check_id_unique(id: u32, db_conn: &DatabaseConnection) -> bool {
    let query = Article::query().filter(Filter::new(Comparison::field("id").equals(id)));
    let art = Article::get(query, db_conn).await.unwrap();
    if art.is_empty() {
        true
    } else {
        false
    }
}
