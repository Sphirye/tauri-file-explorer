use std::fs;
use std::path::PathBuf;

#[derive(serde::Serialize)]
struct FileEntry {
    name: String,
    is_dir: bool,
}

#[tauri::command]
fn read_dir(path: String) -> Result<Vec<FileEntry>, String> {
    let entries = fs::read_dir(PathBuf::from(&path))
        .map_err(|e| format!("No se pudo leer el directorio: {}", e))?;

    let mut result = Vec::new();

    for entry in entries {
        if let Ok(entry) = entry {
            let metadata = entry.metadata().map_err(|e| e.to_string())?;
            result.push(FileEntry {
                name: entry
                    .file_name()
                    .into_string()
                    .unwrap_or_else(|_| "<Nombre invalido".to_string()),
                is_dir: metadata.is_dir(),
            })
        }
    }

    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![read_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
