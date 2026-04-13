use std::path::PathBuf;

pub struct AppState {
    pub db_path: PathBuf,
    pub app_data_dir: PathBuf,
}