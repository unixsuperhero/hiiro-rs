use std::path::PathBuf;
use std::ffi::{OsStr, OsString};
use std::os::unix::prelude::*;

struct Paths {
    pub raw_paths: OsString,
    pub paths: Vec<PathBuf>,
}

impl Paths {
    pub fn default() -> Paths {
        let env_var = std::env::var_os("PATH");

        Self::new(env_var.unwrap())
    }

    pub fn new(list: OsString) -> Paths {
        let paths = std::env::split_paths(&list).collect();

        Self {
            raw_paths: list,
            paths: paths,
        }
    }

    pub fn find(self, name: &str) -> Option<PathBuf> {
        self.possible_bins(name)
            .iter()
            .find(|pbuf| Self::is_executable(pbuf.to_path_buf()))
            .cloned()
    }

    pub fn possible_bins(self, name: &str) -> Vec<PathBuf> {
        self.paths
            .into_iter()
            .map(|pbuf| pbuf.join(&name))
            .collect()
    }

    pub fn is_executable(path: PathBuf) -> bool {
        std::fs::metadata(path)
            .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
}

struct CurrentBin {
    path: PathBuf,
    name: String,
}

impl CurrentBin {
    pub fn default() -> Self {
        let mut args = std::env::args();
        let bin_name = args.nth(0).unwrap();
        let pathbuf = PathBuf::from(bin_name.clone());

        Self::from(pathbuf)
    }

    pub fn from(path: PathBuf) -> Self {
        let dup_path = path.clone();
        let name = dup_path.file_name().and_then(OsStr::to_str).unwrap();

        Self {
            path: path,
            name: name.into(),
        }
    }

    pub fn subcmd(self, subcmd: &str) -> String {
        format!("{}-{}{}", self.name, subcmd, std::env::consts::EXE_SUFFIX)
    }
}

pub fn find_external_subcmd(name: &str) -> Option<PathBuf> {
    let paths = Paths::default();
    let current_bin = CurrentBin::default();

    let subcmd_name = current_bin.subcmd(name);

    paths.find(&subcmd_name)
}

