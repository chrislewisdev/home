use std::{
    collections::HashMap, env, fs::{self, DirEntry}, path::{Path, PathBuf}
};

use anyhow::Context;
use pulldown_cmark::Parser;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FrontMatter {
    title: Option<String>,
    description: Option<String>,
}

struct PageMetadata {
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
    let build_path = PathBuf::from("build");
    let mut context: HashMap<&str, &String> = HashMap::new();
    
    let mut posts: Vec<PageMetadata> = Vec::new();
    let posts_path = build_path.join("posts/");
    for entry in gather_md("content/posts")? {
        let post_path = posts_path.join(get_post_stem(&entry)?);
        fs::create_dir_all(&post_path)?;
        
        posts.push(transform(entry, post_path.join("index.html"), &layout, &context)?);
    }

    let directory = generate_blog_directory(&posts);
    context.insert("{{ blog }}", &directory);

    for entry in gather_md("content")? {
        let html_path = entry.path().with_extension("html");
        let filename = html_path.file_name().context("")?;

        transform(entry, build_path.join(filename), &layout, &context)?;
    }

    Ok(())
}

fn get_post_stem(entry: &DirEntry) -> Result<String, anyhow::Error> {
    let path = entry.path();
    let stem = path.file_stem().context("Unable to extract file stem")?;
    let stem_owned = stem.to_str().map(|s| s.to_string()).context("Unable to extract string from file stem")?;
    let stem_trimmed = stem_owned.get(11..).context("Unable to trim date from post stem")?;

    Ok(stem_trimmed.to_string())
}

fn transform(source: DirEntry, dest: PathBuf, layout: &String, context: &HashMap<&str, &String>) -> anyhow::Result<PageMetadata> {
    let src = fs::read_to_string(source.path())?;

    let (md, meta) = extract_metadata(&src).context(format!("Failed to parse metadata for {}\n", source.path().display()))?;

    let mut subcontext: HashMap<&str, &String> = context.clone();
    subcontext.insert("{{ title }}", &meta.title);

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

fn generate_blog_directory(posts: &Vec<PageMetadata>) -> String {
    let mut output = String::new();

    for post in posts {
        output.push_str(format!("<h3>{}</h3><p>{}</p>", post.title, post.description).as_str());
    }

    output
}

fn extract_metadata<'a>(src: &'a str) -> anyhow::Result<(&'a str, PageMetadata)> {
    let mut meta = PageMetadata {
        title: String::from(""),
        description: String::from("")
    };

    if !src.starts_with("---") {
        return Ok((src, meta));
    }

    let slices: Vec<&str> = src.splitn(3, "---").collect();
    if slices.len() == 2 {
        // There was no closing '---', so just return the whole string
        return Ok((src, meta));
    }

    let toml = slices.get(1).unwrap();
    let front_matter: FrontMatter = toml::from_str(toml)?;
    meta.title = front_matter.title.unwrap_or_default();
    meta.description = front_matter.description.unwrap_or_default();

    return Ok((slices.get(2).unwrap(), meta));
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
