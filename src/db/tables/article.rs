use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;

use crate::db::tables::user;
use hooksaurus_core::datetimes::{deserialize_dt, serialize_dt};

#[derive(serde::Deserialize, serde::Serialize)]
// Just trying this out to avoid the tautology of `ArticleBody<Article>`
pub struct ArticleBody<T = Article> {
    pub article: T,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TagsBody {
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateArticle {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub tag_list: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub author: user::ProfileInfo,
    #[serde(deserialize_with = "deserialize_dt", serialize_with = "serialize_dt")]
    pub created_at: OffsetDateTime,
    #[serde(deserialize_with = "deserialize_dt", serialize_with = "serialize_dt")]
    pub updated_at: OffsetDateTime,
}

pub struct ArticleFromQuery {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub favorited: bool,
    pub favorites_count: i64,
    pub author_username: String,
    pub author_bio: String,
    pub author_image: Option<String>,
    pub following_author: bool,
}

impl ArticleFromQuery {
    pub fn into_article(self) -> Article {
        Article {
            slug: self.slug,
            title: self.title,
            description: self.description,
            body: self.body,
            tag_list: self.tag_list,
            created_at: self.created_at,
            updated_at: self.updated_at,
            // favorited: self.favorited,
            // favorites_count: self.favorites_count,
            author: user::ProfileInfo {
                username: self.author_username,
                bio: self.author_bio,
                image: self.author_image,
                // following: self.following_author,
            },
        }
    }
}

// Collections

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ListArticlesQuery {
    // Theoretically we could allow filtering by multiple tags, e.g. `/api/articles?tag=Rust&tag=SQL`
    // But the Realworld spec doesn't mention that so we're not doing it.
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,

    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// This is technically a subset of `ListArticlesQuery` so we could do some composition
// but it doesn't really save any lines of code and would make these fields slightly less intuitive
// to access in `list_articles()`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FeedArticlesQuery {
    // See comment on these fields in `ListArticlesQuery` above.
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MultipleArticlesBody {
    pub articles: Vec<Article>,

    // This is probably supposed to be the *total* number of rows returned by the current query.
    pub articles_count: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagListInfo {
    pub tags: Vec<String>,
}