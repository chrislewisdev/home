use std::{
    collections::HashMap, env, fs::{self, DirEntry}, path::{Path, PathBuf}
};

use anyhow::Context;
use chrono::{Datelike, NaiveDate, Utc};
use pulldown_cmark::Parser;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FrontMatter {
    title: Option<String>,
    description: Option<String>,
}

struct PageMetadata {
    stem: String,
    date: NaiveDate,
    title: String,
    description: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = cli(args) {
        eprintln!("Error: {:#}", e);
    }
}

fn cli(args: Vec<String>) -> anyhow::Result<()> {
    if args.get(1).is_some_and(|v| v == "clean") {
        clean()?;
    } else {
        generate()?;
    }

    Ok(())
}

fn clean() -> anyhow::Result<()> {
    fs::remove_dir_all("build")?;

    Ok(())
}

fn generate() -> anyhow::Result<()> {
    fs::create_dir_all("build/assets")?;
    copy_files_recursive("assets", PathBuf::from("build/assets"))?;

    let layout = fs::read_to_string("content/layout.html")?;
    let page_layout = layout.replace("{{ body }}", fs::read_to_string("content/page.html")?.as_str());
    let post_layout = layout.replace("{{ body }}", fs::read_to_string("content/post.html")?.as_str());

    let build_path = PathBuf::from("build");
    let mut context: HashMap<&str, &String> = HashMap::new();
    
    let mut posts: Vec<PageMetadata> = Vec::new();
    let posts_path = build_path.join("posts/");
    for entry in gather_md("content/posts")? {
        let file_stem = get_file_stem(&entry)?;
        let stem = get_post_stem(&file_stem)?;
        let date = get_post_date(&file_stem)?;
        let post_path = posts_path.join(&stem);
        fs::create_dir_all(&post_path)?;
        
        posts.push(transform(entry, post_path.join("index.html"), stem, date, &post_layout, &context)?);
    }

    posts.sort_by(|a, b| b.date.cmp(&a.date));
    let blog_directory = generate_blog_directory(&posts, "posts");
    let recent_posts = generate_short_blog_directory(&posts, "posts", 3);
    context.insert("{{ blog }}", &blog_directory);
    context.insert("{{ recent_posts }}", &recent_posts);

    let mut projects: Vec<PageMetadata> = Vec::new();
    let projects_path = build_path.join("projects/");
    for entry in gather_md("content/projects")? {
        let file_stem = get_file_stem(&entry)?;
        let stem = get_post_stem(&file_stem)?;
        let date = get_post_date(&file_stem)?;
        let project_path = projects_path.join(&stem);
        fs::create_dir_all(&project_path)?;
        
        projects.push(transform(entry, project_path.join("index.html"), stem, date, &page_layout, &context)?);
    }

    projects.sort_by(|a, b| b.date.cmp(&a.date));
    let projects_directory = generate_blog_directory(&projects, "projects");
    let recent_projects = generate_short_blog_directory(&projects, "projects", 3);
    context.insert("{{ showcase }}", &projects_directory);
    context.insert("{{ recent_projects }}", &recent_projects);

    let time = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
    context.insert("{{ cachebuster }}", &time);

    for entry in gather_md("content")? {
        let stem = get_file_stem(&entry)?;
        let page_path = if stem != "index" {
            build_path.join(&stem)
        } else {
            build_path.clone()
        };
        fs::create_dir_all(&page_path)?;

        transform(entry, page_path.join("index.html"), stem, Utc::now().date_naive(), &page_layout, &context)?;
    }

    Ok(())
}

fn get_file_stem(entry: &DirEntry) -> Result<String, anyhow::Error> {
    let path = entry.path();
    let stem = path.file_stem().context("Unable to extract file stem")?;
    let stem_owned = stem.to_str().map(|s| s.to_string()).context("Unable to extract string from file stem")?;

    Ok(stem_owned)
}

fn get_post_stem(file_stem: &String) -> Result<String, anyhow::Error> {
    let stem_trimmed = file_stem.get(11..).context("Unable to trim date from post stem")?;

    Ok(stem_trimmed.to_string())
}

fn get_post_date(file_stem: &String) -> Result<NaiveDate, anyhow::Error> {
    let date_string = file_stem.get(0..11).context("Unable to extract date from post stem")?;
    let date_parts: Vec<_> = date_string.split("-").collect();
    let year = date_parts.get(0).context("Missing year in date")?.parse::<i32>()?;
    let month = date_parts.get(1).context("Missing month in date")?.parse::<u32>()?;
    let day = date_parts.get(2).context("Missing day in date")?.parse::<u32>()?;

    NaiveDate::from_ymd_opt(year, month, day).context("Attempted to create invalid date")
}

fn transform(source: DirEntry, dest: PathBuf, stem: String, date: NaiveDate, layout: &String, context: &HashMap<&str, &String>) -> anyhow::Result<PageMetadata> {
    let src = fs::read_to_string(source.path())?;

    let (md, front_matter) = extract_metadata(&src).context(format!("Failed to parse metadata for {}\n", source.path().display()))?;
    let meta = PageMetadata {
        stem,
        date,
        title: front_matter.title.unwrap_or_default(),
        description: front_matter.description.unwrap_or_default(),
    };

    let mut subcontext: HashMap<&str, &String> = context.clone();
    subcontext.insert("{{ title }}", &meta.title);

    let date_formatted = date.format("%d %B %C%y").to_string();
    subcontext.insert("{{ date }}", &date_formatted);

    fs::write(dest, render(md, layout, &subcontext)?)?;
    
    Ok(meta)
}

fn render(markdown: &str, layout: &String, context: &HashMap<&str, &String>) -> anyhow::Result<String> {
    let parser = Parser::new(markdown);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    let mut rendered = layout.clone().replace("{{ content }}", &html);

    for (key, value) in context {
        rendered = rendered.replace(key, value);
    }

    Ok(rendered)
}

fn generate_blog_directory(posts: &Vec<PageMetadata>, base_url: &str) -> String {
    let mut output = String::new();
    // Remind me to make this value bigger when I'm nearly 8000 years old :)
    let mut year_tracker = 10000;

    for post in posts {
        // Output a heading for every new year category
        // I was originally going to solve this by sorting the posts into groups,
        // but Rust is making me painfully aware of the increased cost of that approach vs this.
        let post_year = post.date.year();
        if post_year != year_tracker {
            output.push_str(format!("<h2>&gt; {}</h2>", post_year).as_str());
            year_tracker = post_year;
        }

        let url = format!("/{base_url}/{}", post.stem);
        output.push_str(format!("<h3><a href=\"{}\">{}</a></h3><p>{}</p>", url, post.title, post.description).as_str());
    }

    output
}

fn generate_short_blog_directory(posts: &Vec<PageMetadata>, base_url: &str, count: usize) -> String {
    let mut output = String::new();

    for post in posts.iter().take(count) {
        let url = format!("/{base_url}/{}", post.stem);
        output.push_str(format!("<h3><a href=\"{}\">{}</a></h3><p>{}</p>", url, post.title, post.description).as_str());
    }

    output
}

fn extract_metadata<'a>(src: &'a str) -> anyhow::Result<(&'a str, FrontMatter)> {
    let default_matter = FrontMatter {
        title: Option::None,
        description: Option::None
    };

    if !src.starts_with("---") {
        return Ok((src, default_matter));
    }

    let slices: Vec<&str> = src.splitn(3, "---").collect();
    if slices.len() == 2 {
        // There was no closing '---', so just return the whole string
        return Ok((src, default_matter));
    }

    let toml = slices.get(1).unwrap();
    let front_matter: FrontMatter = toml::from_str(toml)?;

    return Ok((slices.get(2).unwrap(), front_matter));
}

fn gather_md<P>(from: P) -> anyhow::Result<Vec<DirEntry>>
where
    P: AsRef<Path>,
{
    Ok(fs::read_dir(from)?
        .filter_map(|result| result.ok())
        .filter(|entry| {
            entry.path().extension().is_some_and(|ext| ext == "md")
        })
        .collect::<Vec<_>>())
}

fn copy_files_recursive<P>(from: P, to: PathBuf) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_file() {
            fs::copy(entry.path(), to.join(entry.file_name()))?;
        } else if file_type.is_dir() {
            let destination = to.join(entry.file_name());
            fs::create_dir_all(&destination)?;
            copy_files_recursive(entry.path(), destination)?;
        }
    }

    Ok(())
}
