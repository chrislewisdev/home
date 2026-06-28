use std::{fs, path::PathBuf};

use anyhow::Context;
use chrono::{NaiveDate, TimeZone, Utc};
use serde::Serialize;

use crate::PageMetadata;

#[derive(Serialize)]
#[serde(rename = "feed")]
struct Feed {
    #[serde(rename = "@xmlns")]
    xmlns: String,
    id: String,
    title: String,
    updated: String,
    author: Author,
    link: Link,
    entry: Vec<Entry>,
}

#[derive(Serialize)]
struct Link {
    #[serde(rename = "@href")]
    href: String,
}

#[derive(Serialize)]
struct Author {
    name: String,
}

#[derive(Serialize)]
struct Entry {
    id: String,
    title: String,
    summary: String,
    updated: String,
    content: Content,
}

#[derive(Serialize)]
struct Content {
    #[serde(rename = "@src")]
    src: String,
    #[serde(rename = "@type")]
    content_type: String,
}

impl Feed {
    fn new(posts: &Vec<PageMetadata>, updated_date: String) -> Feed {
        Feed {
            xmlns: "http://www.w3.org/2005/Atom".to_string(),
            id: "https://staticlinkage.dev/".to_string(),
            title: "staticlinkage's blog".to_string(),
            updated: updated_date,
            link: Link {
                href: "https://staticlinkage.dev/blog".to_string(),
            },
            author: Author {
                name: "Chris Lewis-Hou".to_string(),
            },
            entry: posts.iter().map(|p| Entry::from(p)).collect(),
        }
    }
}

impl Entry {
    fn from(post: &PageMetadata) -> Entry {
        let uri = format!("https://staticlinkage.dev/posts/{}", post.stem);
        let updated = format_date(&post.date).expect("Failed to format post date");

        Entry {
            id: uri.clone(),
            title: post.title.clone(),
            summary: post.description.clone(),
            updated,
            content: Content {
                src: uri.clone(),
                content_type: "text/html".to_string(),
            },
        }
    }
}

pub fn generate_feed(path: PathBuf, posts: &Vec<PageMetadata>) -> anyhow::Result<()> {
    let most_recent_post = posts
        .get(0)
        .context("Must provide at least one post to generate_feed")?;
    let updated = format_date(&most_recent_post.date)?;

    let f = Feed::new(posts, updated);

    let xml = quick_xml::se::to_string(&f)?;

    fs::write(path, xml)?;

    Ok(())
}

fn format_date(date: &NaiveDate) -> anyhow::Result<String> {
    let date_time = date
        .and_hms_opt(12, 0, 0)
        .context("Failed to add hms to date")?;
    // Need to convert date to UTC otherwise the format panics for some reason!!!;
    let utc_date = Utc
        .from_local_datetime(&date_time)
        .latest()
        .context("Failed to convert to UTC date")?;

    Ok(format!("{}", utc_date.format("%+")))
}
