use std::path::PathBuf;

use super::ImageAction;

pub(crate) trait PathHelper: Sized {
    fn modified(&self, act: &ImageAction, itered: usize) -> Self;
}

impl PathHelper for PathBuf {
    fn modified(&self, act: &ImageAction, itered: usize) -> Self {
        let extension = self.extension().unwrap().to_str().unwrap();
        let stem = self.file_stem().unwrap().to_str().unwrap();
        let act = urlencoding::encode(&act.to_string()).to_string();

        let modified = format!("{}-{}-{}.{}", stem, act, itered, extension);

        self.parent().unwrap().join(modified)
    }
}
