/// In order to view content from local rss files we need to cache it somewhere. This data cannot
/// live in a tempfile because the file may not live long enough for the web browser to open it.
use std::{fs, fs::File, io::Write, path::PathBuf};
use anyhow::Result;

/// A cache in which to store RSS entry content from files for the lifecyle of the Russ
#[derive(Debug)]
pub struct Cache {
    cache_dir: PathBuf,
}

impl Cache {
    pub fn new() -> Result<Self> {
        let proj_dir =
            directories::ProjectDirs::from("", "", "russ").expect("Unable to find home directory");
        let cache_dir = directories::ProjectDirs::cache_dir(&proj_dir).to_path_buf();
        fs::create_dir_all(&cache_dir)?;
        Ok(Self { cache_dir })
    }

    /// Empties out the cache directory
    pub fn clear_cache(&self) -> Result<()> {
        fs::remove_dir(&self.cache_dir)?;
        fs::create_dir_all(&self.cache_dir)?;
        Ok(())
    }

    /// Caches content in a file in the using the given fname if it does not currently exist in the
    /// cache.
    /// NOTE: if the content is modified and the file is already cached it WILL NOT be re-cached.
    pub fn cache_as_file(&self, fname: &str, content: &str) -> Result<()> {
        let cache_file_path = self.cache_dir.join(fname);
        let mut file = File::create(&cache_file_path).map_err(|e| {
            anyhow::anyhow!(format!(
                "could not cache the rss content as file {:?}: {:?}",
                cache_file_path, e
            ))
        })?;
        write!(file, "{}", content)?;
        Ok(())
    }

    /// Check if the cache contains the given file given its name
    pub fn contains(&self, fname: &str) -> bool {
        self.cache_dir.join(fname).exists()
    }
}

impl Drop for Cache {
    fn drop(&mut self) {
        // The cache should be cleared when the application owning it is closed
        // TODO: test what happens if the cache is destroyed during its lifetime. We dont want it
        // panicking on drop!!
        self.clear_cache().expect("Couldn't clear the cache!");
    }
}
