use std::{
    env,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
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
        // e.
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
    for entry in gather_md("content")? {
        let html_path = entry.path().with_extension("html");
        let filename = html_path.file_name().context("")?;

        transform(entry, build_path.join(filename), &layout)?;
    }

    let posts_path = build_path.join("posts/");
    for entry in gather_md("content/posts")? {
        let post_path = posts_path.join(get_post_stem(&entry)?);
        fs::create_dir_all(&post_path)?;
        
        transform(entry, post_path.join("index.html"), &layout)?;
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

fn transform(source: DirEntry, dest: PathBuf, layout: &String) -> anyhow::Result<()> {
    let src = fs::read_to_string(source.path())?;

    let (md, meta) = extract_metadata(&src).context(format!("Failed to parse metadata for {}\n", source.path().display()))?;

    let parser = Parser::new(md);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    let rendered = layout.clone()
                    .replace("{{ title }}", &meta.title)
                    .replace("{{ content }}", &html);

    fs::write(dest, rendered)?;
    
    Ok(())
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
