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

// One place that SQLx could still improve upon is when a query wants to return a nested
// object, such as `Article` wants to with the `author` field.
// For 1:1 relations like that, what we usually do is deserialize the nested object as columns
// flattened into the main query, then fixup the structure afterwards.
//
// It's a good chunk of boilerplate but thankfully you usually only have to write it a few
// times across a whole project.
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
    // This was originally `author_following` to match other fields but that's kind of confusing.
    // That made it sound like a flag showing if the author is following the current user
    // but the intent is the other way round.
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

    // `limit` and `offset` are not the optimal way to paginate SQL queries, because the query
    // planner essentially has to fetch the whole dataset first and then cull it afterwards.
    //
    // It's a much better idea to paginate using the value of an indexed column.
    // For articles, that could be `created_at`, keeping `limit` and then repeatedly querying
    // for `created_at < oldest_created_at_of_previous_query`.
    //
    // Since the spec doesn't return a JSON array at the top level, you could have a `next`
    // field after `articles` that is the URL that the frontend should fetch to get the next page in
    // the ordering, so the frontend doesn't even need to care what column you're using to paginate.
    //
    // However, this is what the Realworld spec calls for.
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
    //
    // However, that necessitates executing the query twice, once to get the rows we actually
    // want to return and a second time just for the count which by necessity must
    // touch all matching rows--not exactly an efficient process.
    //
    // This combined with the limit/offset parameters suggests the design uses an old-fashioned
    // pagination style with page numbers and uses this number to calculate
    // the total number of pages. (Disclaimer: I have not actually looked at the frontend
    // design to be sure; this is just an educated guess.)
    //
    // Modern applications don't really do this anymore and instead implement some sort
    // of infinite scrolling scheme which plays better with paginating based on the value
    // of a column like described on `limit`/`offset` above.
    //
    // It's also more intuitive for the user as they don't really care which page of results
    // they're on. If they're searching for something, they're going to give up if it's
    // not in the first few results anyway. If they're just browsing then they
    // don't usually care where they are in the total ordering of things, or if they do
    // then the scrollbar is already an intuitive indication of where they're at.
    //
    // The Postman collection doesn't test pagination, so as a cop-out I've decided to just
    // return the count of articles currently being returned, which satisfies the happy-path tests.
    pub articles_count: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagListInfo {
    pub tags: Vec<String>,
}

/// Convert a title string to a slug for identifying an article.
///
/// E.g. `slugify("Doctests are the Bee's Knees") == "doctests-are-the-bees-knees"`
///
// (Sadly, doctests are not run on private functions it seems.)
pub fn slugify(string: &str) -> String {
    const QUOTE_CHARS: &[char] = &['\'', '"'];

    string
        // Split on anything that isn't a word character or quotation mark.
        // This has the effect of keeping contractions and possessives together.
        .split(|c: char| !(QUOTE_CHARS.contains(&c) || c.is_alphanumeric()))
        // If multiple non-word characters follow each other then we'll get empty substrings
        // so we'll filter those out.
        .filter(|s| !s.is_empty())
        .map(|s| {
            // Remove quotes from the substring.
            //
            // This allocation is probably avoidable with some more iterator hackery but
            // at that point we'd be micro-optimizing. This function isn't called all that often.
            let mut s = s.replace(QUOTE_CHARS, "");
            // Make the substring lowercase (in-place operation)
            s.make_ascii_lowercase();
            s
        })
        .join("-")
}

// This fulfills the "at least one unit test" requirement of the Realworld spec.
//
// While opinions vary, in general, we're not big fans of TDD at Launchbadge,
// because often you spend most of your time thinking about how you're going to test your code,
// as opposed to getting the job done. When you're on a client's dime, that's really important.
//
// At the same time, you're making your code more difficult to read and reason about because
// you're forced to separate the code from its dependencies for testing.
//
// For example, most of the handler functions in this API touch the database, which isn't
// conducive to unit testing. Sure, you could mock those database calls out but then there's
// really not whole lot left to test. For what little is left, the logic should ideally
// be self-evident, and then testing is just superfluous.
//
// Of course, testing is still really important. Manually testing the API every time you make
// a change only goes so far, can become really unwieldy, and is easy to forget or neglect
// to do because of that.
//
// I'm personally a big proponent of unit-testing only what makes sense to unit-test,
// such as self-contained functions like `slugify()`. The rest can be covered with integration
// or end-to-end testing, which we do a lot of at Launchbadge. That has the advantage of not
// only covering the API, but the frontend as well.
//
// Fortunately, the Realworld spec comes with an API integration test suite already, although
// in many places it doesn't cover much more than just the happy paths. I wish I had the time
// and energy to help fill that out.
#[test]
fn test_slugify() {
    assert_eq!(
        slugify("Segfaults and You: When Raw Pointers Go Wrong"),
        "segfaults-and-you-when-raw-pointers-go-wrong"
    );

    assert_eq!(
        slugify("Why are DB Admins Always Shouting?"),
        "why-are-db-admins-always-shouting"
    );

    assert_eq!(
        slugify("Converting to Rust from C: It's as Easy as 1, 2, 3!"),
        "converting-to-rust-from-c-its-as-easy-as-1-2-3"
    )
}
