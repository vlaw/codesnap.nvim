use crate::{config::TakeSnapshotParams, path::parse_save_path, snapshot::take_snapshot};
use nvim_oxi::api;

// The function will be called as FFI on Lua side
#[allow(dead_code)]
pub fn save_snapshot(config: TakeSnapshotParams) -> Result<(), api::Error> {
    match &config.save_path {
        Some(path) => {
            if !path.ends_with(".png") {
                return Err(api::Error::Other(
                    "The save_path must ends with .png".to_string(),
                ));
            }

            let pixmap = take_snapshot(config.clone())?;
            let path = parse_save_path(path.to_string())
                .map_err(|err| api::Error::Other(err.to_string()))?;

            pixmap
                .save_png(path)
                .map_err(|err| api::Error::Other(err.to_string()))
        }
        None => Err(api::Error::Other(
            "Cannot find 'save_path' in config".to_string(),
        )),
    }
}
