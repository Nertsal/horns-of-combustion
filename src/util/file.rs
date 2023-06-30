use geng::prelude::{
    anyhow::{self, Context},
    DeserializeOwned,
};

pub async fn load_file<T: DeserializeOwned>(
    path: impl AsRef<std::path::Path>,
) -> anyhow::Result<T> {
    let path = path.as_ref();
    geng::prelude::file::load_detect(path)
        .await
        .context(format!("when loading {:?}", path))
}
