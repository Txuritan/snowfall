use serde::{Deserialize, Serialize};

use juniper::graphql_object;

#[derive(Serialize, Deserialize)]
pub struct PageInfo {
    #[serde(rename = "hasNextPage")]
    pub next_page: bool,
    #[serde(rename = "hasPreviousPage")]
    pub previous_page: bool,
    #[serde(rename = "startCursor")]
    pub start_cursor: Option<String>,
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Bookmark {
    pub id: i32,
    pub title: String,
    pub domain: String,
    pub url: String,

    pub tags: Vec<Tag>,
}

graphql_object!(Bookmark: () |&self| {
    description: "A user's bookmark"

    field id() -> &i32 {
        &self.id
    }

    field title() -> &String {
        &self.title
    }

    field domain() -> &String {
        &self.domain
    }

    field url() -> &String {
        &self.url
    }

    field tags() -> &Vec<Tag> {
        &self.tags
    }
});

#[derive(Serialize, Deserialize)]
pub struct BookmarkConnection {
    #[serde(rename = "pageInfo")]
    page_info: PageInfo,
    edges: Option<Vec<BookmarkEdge>>,
    #[serde(rename = "totalCount")]
    total_count: Option<i32>,
    bookmarks: Option<Vec<Bookmark>>,
}

#[derive(Serialize, Deserialize)]
pub struct BookmarkEdge {
    cursor: String,
    node: Option<Bookmark>,
}

#[derive(Serialize, Deserialize)]
pub struct Collection {
    pub id: i32,
    pub title: String,
}

graphql_object!(Collection: () |&self| {
    description: "A user's collection of bookmark"

    field id() -> &i32 {
        &self.id
    }

    field title() -> &String {
        &self.title
    }
});

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

graphql_object!(Tag: () |&self| {
    description: "A bookmark's tag"

    field id() -> &i32 {
        &self.id
    }

    field name() -> &String {
        &self.name
    }
});

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
}

graphql_object!(User: () |&self| {
    field id() -> &String {
        &self.id
    }

    field username() -> &String {
        &self.username
    }

    field email() -> &String {
        &self.email
    }
});

#[derive(Serialize, Deserialize)]
pub struct Login {}

#[derive(Serialize, Deserialize)]
pub struct Logout {}
