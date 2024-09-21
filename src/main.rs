use std::{
    env, fs::{self}, path::{Path, PathBuf}
};

use anyhow::Context;
use pulldown_cmark::{html, Parser};

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

    for entry in fs::read_dir("content")? {
        let entry = entry?;
        if entry.path().extension().is_some_and(|v| v == "md") {
            let md = fs::read_to_string(entry.path())?;
            let parser = Parser::new(md.as_str());

            let mut htm = String::new();
            html::push_html(&mut htm, parser);
            
            let rendered = layout.clone().replace("{{content}}", &htm);
            let html_path = entry.path().with_extension("html");
            let filename = html_path.file_name().context("")?;
            fs::write(PathBuf::from("build").join(filename), rendered)?;
        }
    }

    Ok(())
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
