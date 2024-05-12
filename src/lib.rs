
pub mod extern_subcmd {
    use std::path::PathBuf;
    use std::os::unix::prelude::*;
    use std::fs;
    use std::env;

    pub fn is_executable(path: PathBuf) -> bool {
        fs::metadata(path)
            .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }

    pub fn possible_bins(bin_name: &str, subcmd: &str) -> Vec<PathBuf> {
        let name = format!("{}-{}{}", bin_name, subcmd, env::consts::EXE_SUFFIX);
        paths()
            .into_iter()
            .map(|pbuf| pbuf.join(&name))
            .collect()
    }

    pub fn paths() -> Vec<PathBuf> {
        let path = env::var_os("PATH").unwrap();
        env::split_paths(&path).collect()
    }

    pub fn subcmd_path<'a>(name: &str) -> Option<PathBuf> {
        let paths: Vec<PathBuf> = possible_bins("h", &name);
        match paths.iter().find(|pbuf| is_executable(pbuf.clone().clone()) ) {
            Some(p) => Some(p.clone()),
            None => None
        }
    }
}
