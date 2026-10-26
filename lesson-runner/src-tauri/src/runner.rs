use serde::Serialize;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tauri::{AppHandle, Emitter, State};

/// The step that is running, if any. Shared with the commands through Tauri state.
#[derive(Default)]
pub struct RunState {
    child: Mutex<Option<Arc<Mutex<Child>>>>,
    stdin: Mutex<Option<ChildStdin>>,
}

#[derive(Serialize, Clone)]
struct OutputEvent {
    stream: &'static str,
    text: String,
}

#[derive(Serialize, Clone)]
struct FinishedEvent {
    code: Option<i32>,
}

/// Runs `cargo run` for a step. With `source`, the step is first copied to a
/// temporary folder and its `src/main.rs` replaced, so the repository is untouched.
pub fn start(
    app: AppHandle,
    state: State<'_, RunState>,
    dir: String,
    source: Option<String>,
    backtrace: bool,
) -> Result<(), String> {
    stop(&state);

    let mut command = Command::new("cargo");
    if backtrace {
        // Set for this run only, so panics show a backtrace without changing the user's environment.
        command.env("RUST_BACKTRACE", "1");
    }
    command
        .arg("run")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    match source {
        None => {
            command.current_dir(&dir);
        }
        Some(text) => {
            let work = prepare_work_copy(Path::new(&dir), &text)?;
            command.current_dir(&work).env("CARGO_TARGET_DIR", work_root().join("target"));
        }
    }

    let mut child = command
        .spawn()
        .map_err(|e| format!("Could not start cargo: {e}"))?;

    let stdin = child.stdin.take();
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let child = Arc::new(Mutex::new(child));
    *state.stdin.lock().unwrap() = stdin;
    *state.child.lock().unwrap() = Some(Arc::clone(&child));

    let mut readers = Vec::new();
    if let Some(out) = stdout {
        readers.push(pump(app.clone(), out, "stdout"));
    }
    if let Some(err) = stderr {
        readers.push(pump(app.clone(), err, "stderr"));
    }

    wait(app, child, readers);
    Ok(())
}

/// Sends text to the running program's standard input.
pub fn send(state: &RunState, text: &str) -> Result<(), String> {
    let mut guard = state.stdin.lock().unwrap();
    let stdin = guard.as_mut().ok_or_else(|| "No program is running".to_string())?;
    stdin.write_all(text.as_bytes()).map_err(|e| e.to_string())?;
    stdin.flush().map_err(|e| e.to_string())
}

/// Stops the running program, if there is one.
pub fn stop(state: &RunState) {
    *state.stdin.lock().unwrap() = None;
    let child = state.child.lock().unwrap().take();
    if let Some(child) = child {
        let _ = child.lock().unwrap().kill();
    }
}

fn work_root() -> PathBuf {
    std::env::temp_dir().join("lesson-runner")
}

/// Copies the step (without its `target` folder) and replaces its `src/main.rs`.
fn prepare_work_copy(step: &Path, text: &str) -> Result<PathBuf, String> {
    let name = step
        .file_name()
        .ok_or_else(|| "Invalid step folder".to_string())?;
    let work = work_root().join("work").join(name);
    copy_dir(step, &work)?;
    fs::write(work.join("src").join("main.rs"), text)
        .map_err(|e| format!("Could not save the edited code: {e}"))?;
    Ok(work)
}

fn copy_dir(from: &Path, to: &Path) -> Result<(), String> {
    fs::create_dir_all(to).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(from).map_err(|e| e.to_string())?.flatten() {
        let name = entry.file_name();
        if name.to_string_lossy() == "target" {
            continue;
        }
        let path = entry.path();
        let dest = to.join(&name);
        if path.is_dir() {
            copy_dir(&path, &dest)?;
        } else {
            fs::copy(&path, &dest).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// Reads a stream in chunks, so a prompt with no newline (such as "Enter a number: ")
/// shows at once.
///
/// Cargo's "Running ..." line may arrive in two reads (the text, then its newline), so
/// the current line is kept between reads. When a line that starts the program is
/// complete, a separate "gap" event adds two blank lines after it.
fn pump<R: Read + Send + 'static>(
    app: AppHandle,
    mut reader: R,
    stream: &'static str,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut buf = [0u8; 1024];
        let mut line = String::new();
        loop {
            match reader.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    let text = String::from_utf8_lossy(&buf[..n]).into_owned();
                    for piece in text.split_inclusive('\n') {
                        let _ = app.emit(
                            "run-output",
                            OutputEvent {
                                stream,
                                text: piece.to_string(),
                            },
                        );
                        line.push_str(piece);
                        if piece.ends_with('\n') {
                            if line.trim_start().starts_with("Running `") {
                                let _ = app.emit(
                                    "run-output",
                                    OutputEvent {
                                        stream: "gap",
                                        text: "\n\n".to_string(),
                                    },
                                );
                            }
                            line.clear();
                        }
                    }
                }
            }
        }
    })
}

/// Waits for the program to exit, then for its output to be read, then reports the exit code.
fn wait(app: AppHandle, child: Arc<Mutex<Child>>, readers: Vec<JoinHandle<()>>) {
    thread::spawn(move || {
        let code = loop {
            let status = child.lock().unwrap().try_wait();
            match status {
                Ok(Some(status)) => break status.code(),
                Ok(None) => thread::sleep(Duration::from_millis(100)),
                Err(_) => break None,
            }
        };
        for reader in readers {
            let _ = reader.join();
        }
        let _ = app.emit("run-finished", FinishedEvent { code });
    });
}
