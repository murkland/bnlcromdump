#![windows_subsystem = "windows"]

fn dump_bnlc_rom_archives(
    lc_path: &std::path::Path,
    output_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let data_path = lc_path.join("exe").join("data");
    let read_dir = std::fs::read_dir(&data_path)?;
    for entry in read_dir {
        let entry = entry?;

        if entry.path().file_name() == Some(&std::ffi::OsStr::new("exe.dat"))
            || entry.path().extension() != Some(&std::ffi::OsStr::new("dat"))
        {
            continue;
        }

        dump_bnlc_rom_archive(&entry.path(), output_path)?;
    }

    Ok(())
}

fn dump_bnlc_rom_archive(
    path: &std::path::Path,
    output_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open(path)?;
    let mut za = zip::ZipArchive::new(f)?;

    for i in 0..za.len() {
        let mut entry = za.by_index(i).unwrap();

        let entry_path = if let Some(entry_path) = entry.enclosed_name() {
            entry_path.to_owned()
        } else {
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

        let mut output_f = std::fs::File::create(&output_path.join(filename))?;
        std::io::copy(&mut entry, &mut output_f)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = (|| {
        let mut steamdir = if let Some(steamdir) = steamlocate::SteamDir::locate() {
            steamdir
        } else {
            native_dialog::MessageDialog::new()
                .set_type(native_dialog::MessageType::Warning)
                .set_title("Steam installation was not detected. Steamのインストールが検出されませんでした。")
                .set_text("You must have Mega Man Battle Network Legacy Collection installed via Steam to use this tool. このツールを使用するには、Steamで「ロックマンエグゼ アドバンスドコレクション」がインストールされる必要があります。")
                .show_alert()?;
            return Ok(());
        };

        let apps = steamdir.apps();

        let paths = [
            apps.get(&1798010)
                .and_then(|v| v.as_ref())
                .map(|app| app.path.clone()),
            apps.get(&1798020)
                .and_then(|v| v.as_ref())
                .map(|app| app.path.clone()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

        if paths.is_empty() {
            native_dialog::MessageDialog::new()
                .set_type(native_dialog::MessageType::Warning)
                .set_title("Mega Man Battle Network Legacy Collection was not detected. 「ロックマンエグゼ アドバンスドコレクション」は検出されませんでした。")
                .set_text("You must have Mega Man Battle Network Legacy Collection installed via Steam to use this tool. このツールを使用するには、Steamで「ロックマンエグゼ アドバンスドコレクション」がインストールされる必要があります。")
                .show_alert()?;
            return Ok(());
        }

        let output_path =
            if let Some(output_path) = native_dialog::FileDialog::new().show_open_single_dir()? {
                output_path
            } else {
                return Ok(());
            };

        for path in paths {
            dump_bnlc_rom_archives(&path, &output_path)?;
        }

        Ok::<_, Box<dyn std::error::Error>>(())
    })() {
        native_dialog::MessageDialog::new()
            .set_type(native_dialog::MessageType::Error)
            .set_title(&format!("An error occurred. エラーが発生しました"))
            .set_text(&format!("{}", e))
            .show_alert()?;
    }
    Ok(())
}
