mod discovery;
mod runner;

use runner::RunState;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder, WindowEvent};

/// The lesson currently shown in the main window, for the pop-out window to read.
#[derive(Serialize, Clone, Default)]
pub struct LessonData {
    pub title: String,
    pub text: String,
}

#[derive(Default)]
pub struct LessonState {
    current: Mutex<LessonData>,
    popped: AtomicBool,
}

/// The theme shared by every window: true for light, false for dark.
#[derive(Default)]
pub struct ThemeState {
    light: AtomicBool,
}

#[tauri::command]
fn set_theme(app: AppHandle, state: State<'_, ThemeState>, light: bool) {
    state.light.store(light, Ordering::SeqCst);
    let _ = app.emit("theme-changed", light);
}

#[tauri::command]
fn get_theme(state: State<'_, ThemeState>) -> bool {
    state.light.load(Ordering::SeqCst)
}

#[tauri::command]
fn list_chapters() -> Vec<discovery::Chapter> {
    discovery::discover()
}

/// The `.rs` files in a step's `src` folder, as paths relative to `src` with `/` separators. `main.rs` comes first.
#[tauri::command]
fn list_source_files(path: String) -> Vec<String> {
    let src = std::path::Path::new(&path).join("src");
    let mut files = Vec::new();
    collect_rs_files(&src, &src, &mut files);
    files.sort_by(|a, b| (a != "main.rs").cmp(&(b != "main.rs")).then_with(|| a.cmp(b)));
    files
}

fn collect_rs_files(root: &std::path::Path, dir: &std::path::Path, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            collect_rs_files(root, &p, out);
        } else if p.extension().is_some_and(|e| e == "rs") {
            if let Ok(rel) = p.strip_prefix(root) {
                out.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
    }
}

/// The text of a `.rs` file in a step's `src` folder, for the Source tab.
#[tauri::command]
fn read_source(path: String, file: String) -> Result<String, String> {
    if file.contains("..") || file.starts_with('/') || file.starts_with('\\') || !file.ends_with(".rs") {
        return Err(format!("Not a source file: {file}"));
    }
    let full = std::path::Path::new(&path).join("src").join(&file);
    std::fs::read_to_string(&full).map_err(|e| format!("Could not read {}: {e}", full.display()))
}

/// The text of a step's `lesson.md`, or an empty string if it has none.
#[tauri::command]
fn read_lesson(path: String) -> String {
    std::fs::read_to_string(std::path::Path::new(&path).join("lesson.md")).unwrap_or_default()
}

/// A text file from a step folder that the lesson pane may show. Only `lesson.md` and
/// `solution.md` are allowed.
#[tauri::command]
fn read_step_file(path: String, file: String) -> String {
    if file != "lesson.md" && file != "solution.md" {
        return String::new();
    }
    std::fs::read_to_string(std::path::Path::new(&path).join(file)).unwrap_or_default()
}

/// Records the lesson on screen and tells any open pop-out window to show it.
#[tauri::command]
fn set_lesson(app: AppHandle, state: State<'_, LessonState>, title: String, text: String) {
    let data = LessonData { title, text };
    *state.current.lock().unwrap() = data.clone();
    let _ = app.emit("lesson-updated", data);
}

#[tauri::command]
fn get_lesson(state: State<'_, LessonState>) -> LessonData {
    state.current.lock().unwrap().clone()
}

/// Opens the lesson in its own window, or brings the existing one to the front.
/// This must stay `async`: on Windows, creating a window from a synchronous command can deadlock.
#[tauri::command]
async fn open_lesson_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("lesson") {
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }
    let window = WebviewWindowBuilder::new(&app, "lesson", WebviewUrl::App("lesson.html".into()))
        .title("Lesson")
        .inner_size(900.0, 720.0)
        .build()
        .map_err(|e| e.to_string())?;

    // However the window closes (the dock button, its close box, or the main window closing),
    // the lesson is docked again.
    let handle = app.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::Destroyed = event {
            handle.state::<LessonState>().popped.store(false, Ordering::SeqCst);
            let _ = handle.emit("lesson-docked", ());
        }
    });

    app.state::<LessonState>().popped.store(true, Ordering::SeqCst);
    let _ = app.emit("lesson-popped", ());
    Ok(())
}

/// Closes the pop-out window. The lesson returns to the main window.
#[tauri::command]
async fn dock_lesson(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("lesson") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Asks the main window to navigate to a named chapter, section, or step. Links in the pop-out use this too.
#[tauri::command]
fn navigate_to(app: AppHandle, name: String) {
    let _ = app.emit("goto", name);
}

/// Opens a web link in the default browser. Only web and mail links are allowed.
#[tauri::command]
fn open_external(url: String) -> Result<(), String> {
    let allowed = url.starts_with("https://") || url.starts_with("http://") || url.starts_with("mailto:");
    if !allowed {
        return Err(format!("Not opening {url}"));
    }
    open::that(&url).map_err(|e| e.to_string())
}

#[tauri::command]
fn run_step(
    app: tauri::AppHandle,
    state: State<'_, RunState>,
    path: String,
    source: Option<String>,
    backtrace: bool,
    test: bool,
) -> Result<(), String> {
    runner::start(app, state, path, source, backtrace, test)
}

#[tauri::command]
fn send_input(state: State<'_, RunState>, text: String) -> Result<(), String> {
    runner::send(&state, &text)
}

#[tauri::command]
fn stop_run(state: State<'_, RunState>) {
    runner::stop(&state);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(RunState::default())
        .manage(LessonState::default())
        .manage(ThemeState::default())
        .setup(|app| {
            // The title-bar icon. The installed program's icon comes from tauri.conf.json.
            if let Some(window) = app.get_webview_window("main") {
                window.set_icon(tauri::include_image!("icons/icon.png"))?;

                // Closing the main window also closes the pop-out lesson, so the app can exit.
                let handle = app.handle().clone();
                window.on_window_event(move |event| {
                    if let WindowEvent::Destroyed = event {
                        let handle = handle.clone();
                        tauri::async_runtime::spawn(async move {
                            if let Some(lesson) = handle.get_webview_window("lesson") {
                                let _ = lesson.close();
                            }
                        });
                    }
                });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_chapters,
            read_source,
            list_source_files,
            read_lesson,
            read_step_file,
            set_lesson,
            get_lesson,
            open_lesson_window,
            dock_lesson,
            open_external,
            navigate_to,
            set_theme,
            get_theme,
            run_step,
            send_input,
            stop_run
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
