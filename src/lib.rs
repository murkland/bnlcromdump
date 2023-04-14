#[cfg(windows)]
pub fn find_steam_path() -> Option<std::path::PathBuf> {
    let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);

    Some(
        if let Some(install_path) = [
            "SOFTWARE\\Valve\\Steam",
            "SOFTWARE\\Wow6432Node\\Valve\\Steam",
        ]
        .into_iter()
        .flat_map(|path| {
            hklm.open_subkey(path).ok().and_then(|subkey| {
                subkey
                    .get_value::<std::ffi::OsString, _>("InstallPath")
                    .ok()
                    .map(std::path::PathBuf::from)
            })
        })
        .next()
        {
            install_path
        } else {
            return None;
        },
    )
}

#[cfg(not(windows))]
pub fn find_steam_path() -> Option<std::path::PathBuf> {
    None
}
