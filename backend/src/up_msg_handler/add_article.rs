//! Defines functions used for adding an article to database.
use crate::Article;
use aragog::query::{Comparison, Filter};
use aragog::{DatabaseRecord, Record};
use rand::Rng;
use shared::DownMsg;

/// The handler for adding article to DB. Returns a DownMsg containing an empty message.
///
/// # Arguments
/// * `title` - A String holding the title of the article.
/// * `content` - A String holding the content of the article.
/// * `author` - A String holding the author of the article.
/// * `tags` - A vector of Strings holding the article's tags.
pub async fn handler(title: String, content: String, author: String, tags: Vec<String>) -> DownMsg {
    create_article_in_db(title, content, author, tags).await;
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
) {
    let conn = crate::init_db().await;
    let db_article = Article::new(
        generate_id().await,
        title,
        content,
        vec![],
        author,
        tags,
        super::get_time(),
        super::get_time(),
    );
    DatabaseRecord::create(db_article, &conn).await.unwrap();
}

/// Returns an id as u32 when the generated id is unique.
async fn generate_id() -> u32 {
    let mut rand = rand::thread_rng();
    let mut id: u32 = 0;
    let mut checking = true;
    while checking {
        id = rand.gen::<u32>();
        if check_id_unique(id.clone()).await {
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
async fn check_id_unique(id: u32) -> bool {
    let conn = crate::init_db().await;

    let query = Article::query().filter(Filter::new(Comparison::field("id").equals(id)));
    let art = Article::get(query, &conn).await.unwrap();
    if art.is_empty() {
        true
    } else {
        false
    }
}
