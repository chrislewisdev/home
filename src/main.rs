use std::{
    env,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use anyhow::Context;
use pulldown_cmark::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = cli(args) {
        eprintln!("Error: {e}");
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
    copy_assets("assets", PathBuf::from("build/assets"))?;

    let layout = fs::read_to_string("content/layout.html")?;

    let build_path = PathBuf::from("build");
    for entry in gather_md("content")? {
        transform(entry, &build_path, &layout)?;
    }

    let posts_path = build_path.join("posts/");
    fs::create_dir_all(&posts_path)?;
    for entry in gather_md("content/posts")? {
        transform(entry, &posts_path, &layout)?;
    }

    Ok(())
}

fn transform(entry: DirEntry, base_path: &PathBuf, layout: &String) -> anyhow::Result<()> {
    let md = fs::read_to_string(entry.path())?;

    let parser = Parser::new(md.as_str());
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    let rendered = layout.clone().replace("{{content}}", &html);
    let html_path = entry.path().with_extension("html");
    let filename = html_path.file_name().context("")?;
    fs::write(base_path.join(filename), rendered)?;
    
    Ok(())
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

fn copy_assets<P>(from: P, to: PathBuf) -> anyhow::Result<()>
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
            copy_assets(entry.path(), destination)?;
        }
    }

    Ok(())
}
