use directories_next::ProjectDirs;
use std::path::PathBuf;
use std::fs::create_dir_all;


struct Project {
    name: String,
    path: String,
}

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "ProjectManager") {
        let config_dir = proj_dirs.data_dir();
        create_dir_all(config_dir)?;
        Ok(config_dir.to_path_buf())
    } else {
        Err("Could not determine the config directory path.".into())
    }
}

pub fn add_project(name: &String) {
    let config_path = get_config_path();

}
