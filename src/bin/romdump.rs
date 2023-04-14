use std::str::FromStr;

fn dump_bnlc_rom_archives(lc_path: &std::path::Path, output_path: &std::path::Path) {
    let data_path = lc_path.join("exe").join("data");
    let read_dir = match std::fs::read_dir(&data_path) {
        Ok(read_dir) => read_dir,
        Err(_) => {
            return;
        }
    };
    for entry in read_dir {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => {
                continue;
            }
        };

        if entry.path().file_name() == Some(&std::ffi::OsStr::new("exe.dat"))
            || entry.path().extension() != Some(&std::ffi::OsStr::new("dat"))
        {
            continue;
        }

        dump_bnlc_rom_archive(&entry.path(), output_path);
    }
}

fn dump_bnlc_rom_archive(path: &std::path::Path, output_path: &std::path::Path) {
    let f = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => {
            log::error!("no such file: {}", path.display());
            return;
        }
    };
    let mut za = match zip::ZipArchive::new(f) {
        Ok(za) => za,
        Err(e) => {
            log::error!("failed to open lc archive {}: {}", path.display(), e);
            return;
        }
    };

    for i in 0..za.len() {
        let mut entry = za.by_index(i).unwrap();

        let entry_path = if let Some(entry_path) = entry.enclosed_name() {
            entry_path.to_owned()
        } else {
            log::error!("failed to read lc archive entry {} {}", path.display(), i);
            continue;
        };

        if entry_path.extension() != Some(&std::ffi::OsStr::new("srl")) {
            continue;
        }

        let filename = format!(
            "{}_{}.gba",
            path.with_extension("")
                .file_name()
                .unwrap()
                .to_string_lossy(),
            entry_path
                .with_extension("")
                .file_name()
                .unwrap()
                .to_string_lossy()
        );
        log::info!("dumping: {}", filename);

        let p = output_path.join(filename);
        let mut output_f = match std::fs::File::create(&p) {
            Ok(f) => f,
            Err(e) => {
                log::error!("failed to create dump target {}: {}", p.display(), e);
                continue;
            }
        };

        if let Err(e) = std::io::copy(&mut entry, &mut output_f) {
            log::error!("failed to copy rom {}: {}", p.display(), e);
            continue;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .filter(Some("romdump"), log::LevelFilter::Info)
        .init();
    let steam_path = bnlctools::find_steam_path().unwrap();
    log::info!("found steam installation: {}", steam_path.display());

    let steamapps_common_path = steam_path.join("steamapps").join("common");

    let output_path = std::path::PathBuf::from_str("roms")?;
    let _ = std::fs::create_dir(&output_path);

    dump_bnlc_rom_archives(
        &steamapps_common_path.join("MegaMan_BattleNetwork_LegacyCollection_Vol1"),
        &output_path,
    );
    dump_bnlc_rom_archives(
        &steamapps_common_path.join("MegaMan_BattleNetwork_LegacyCollection_Vol2"),
        &output_path,
    );
    Ok(())
}
