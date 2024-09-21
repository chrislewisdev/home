use std::{
    env, fs::{self}, path::{Path, PathBuf}
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = cli(args) {
        eprintln!("Error: {e}");
    }
}

fn cli(args: Vec<String>) -> anyhow::Result<()> {
    if args.get(1).is_some_and(|v| v == "clean") {
        println!("Cleaning");
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
