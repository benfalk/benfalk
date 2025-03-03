//! Git2 Playgound
#![allow(dead_code, unused_imports, unused_variables)]

#[cfg(test)]
mod test {
    use anyhow::{Context, Ok, Result};
    use git2::Repository;
    use std::path::PathBuf;

    #[test]
    fn opening_a_repo() -> Result<()> {
        let file = PathBuf::from("crates/async_cb/src/lib.rs");
        let repo = Repository::discover("/home/bfalk/Projects/journal/crates/")?;
        let head = repo.revparse("HEAD")?;
        let id = head.from().unwrap().id();
        let commit = repo.find_commit(id)?;
        let tree = commit.tree()?;
        let entry = tree.get_path(&file)?;
        let blob = entry.to_object(&repo)?.into_blob().unwrap();
        let content = String::from_utf8(blob.content().to_owned())?;
        println!("{content}");
        Ok(())
    }
}
