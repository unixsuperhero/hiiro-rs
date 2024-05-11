use std::env;

pub fn subcmd_bin(bin_name: &str, subcmd: &str) -> String {
    format!("{}-{}{}", bin_name, subcmd, env::consts::EXE_SUFFIX)
}
