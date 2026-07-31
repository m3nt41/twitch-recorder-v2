#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Stdio, Child};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use tauri::Manager;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

static RECORDING_PROCESS: OnceLock<Mutex<Option<Child>>> = OnceLock::new();
static LAST_RECORDING_PATH: OnceLock<Mutex<Option<String>>> = OnceLock::new();

fn get_process_mutex() -> &'static Mutex<Option<Child>> {
    RECORDING_PROCESS.get_or_init(|| Mutex::new(None))
}

fn get_last_path_mutex() -> &'static Mutex<Option<String>> {
    LAST_RECORDING_PATH.get_or_init(|| Mutex::new(None))
}

#[tauri::command]
fn start_recording(
    app_handle: tauri::AppHandle,
    channel: String,
) -> Result<String, String> {
    let proc_mutex = get_process_mutex();
    let path_mutex = get_last_path_mutex();
    
    if let Ok(mut proc_opt) = proc_mutex.lock() {
        if let Some(mut proc) = proc_opt.take() {
            let _ = proc.kill();
        }
    }

    // Buscar el archivo en la carpeta resources
    let sidecar_path = app_handle
        .path()
        .resolve("resources/streamlink-x86_64-pc-windows-msvc.exe", tauri::path::BaseDirectory::Resource)
        .map_err(|e| format!("Error resolviendo ruta: {}", e))?;

    if !sidecar_path.exists() {
        return Err(format!("El archivo de streamlink no se encontró en: {:?}", sidecar_path));
    }

    // Carpeta de grabaciones
    let mut output_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .ok_or_else(|| "No se pudo obtener directorio del exe".to_string())?
        .to_path_buf();
    
    #[cfg(debug_assertions)]
    for _ in 0..3 {
        output_dir.pop();
    }
    output_dir.push("grabaciones");

    fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    let output_file = output_dir.join(format!("{}_{}.mp4", channel, timestamp));
    
    // Guardar la ruta del archivo actual
    if let Ok(mut path_opt) = path_mutex.lock() {
        *path_opt = Some(output_file.to_string_lossy().to_string());
    }

    // Ejecutar Streamlink
    let mut cmd = Command::new(&sidecar_path);
    cmd.args([
        &format!("https://www.twitch.tv/{}", channel),
        "best",
        "-o",
        &output_file.to_string_lossy(),
        "--force",
    ])
    .stdout(Stdio::null())
    .stderr(Stdio::null());

    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let child = cmd.spawn().map_err(|e| format!("Error al ejecutar streamlink: {}", e))?;

    if let Ok(mut proc_opt) = proc_mutex.lock() {
        *proc_opt = Some(child);
    }
    
    Ok(format!("✅ Grabación iniciada: {}", channel))
}

#[tauri::command]
fn stop_recording() -> Result<String, String> {
    let proc_mutex = get_process_mutex();
    let path_mutex = get_last_path_mutex();
    
    // Obtener la ruta del último archivo grabado
    let last_path = if let Ok(path_opt) = path_mutex.lock() {
        path_opt.clone()
    } else {
        None
    };
    
    if let Ok(mut proc_opt) = proc_mutex.lock() {
        if let Some(mut proc) = proc_opt.take() {
            let pid = proc.id();
            
            #[cfg(target_os = "windows")]
            {
                let _ = Command::new("taskkill")
                    .args(["/PID", &pid.to_string(), "/T", "/F"])
                    .creation_flags(CREATE_NO_WINDOW)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn();
            }
            
            #[cfg(not(target_os = "windows"))]
            {
                let _ = proc.kill();
            }
        }
    }
    
    // Limpiar la ruta después de detener
    if let Ok(mut path_opt) = path_mutex.lock() {
        *path_opt = None;
    }
    
    // Retornar mensaje con la ruta del archivo
    if let Some(file_path) = last_path {
        Ok(format!("⏹️ Grabación detenida.\n\n📁 Archivo guardado en:\n{}", file_path))
    } else {
        Ok("️ Grabación detenida.".to_string())
    }
}

#[tauri::command]
fn open_recordings_folder() -> Result<String, String> {
    let mut recordings_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .ok_or_else(|| "No se pudo obtener directorio del exe".to_string())?
        .to_path_buf();
    
    #[cfg(debug_assertions)]
    for _ in 0..3 {
        recordings_dir.pop();
    }
    recordings_dir.push("grabaciones");

    // Crear la carpeta si no existe
    fs::create_dir_all(&recordings_dir).map_err(|e| e.to_string())?;

    // Abrir la carpeta
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(&recordings_dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&recordings_dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&recordings_dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok("📂 Carpeta de grabaciones abierta".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_recording, 
            stop_recording,
            open_recordings_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}