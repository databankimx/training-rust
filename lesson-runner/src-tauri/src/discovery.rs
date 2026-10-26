use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Clone)]
pub struct Step {
    pub name: String,
    pub title: String,
    pub path: String,
    pub has_code: bool,
}

#[derive(Serialize, Clone)]
pub struct Section {
    pub name: String,
    pub title: String,
    pub path: String,
    pub steps: Vec<Step>,
}

#[derive(Serialize, Clone)]
pub struct Chapter {
    pub name: String,
    pub title: String,
    pub path: String,
    pub steps: Vec<Step>,
    pub homework: Vec<Step>,
    pub homework_path: String,
    pub sections: Vec<Section>,
}

/// The training repository root. The runner lives at
/// `<root>/lesson-runner/src-tauri`, so the root is two levels up from this crate.
pub fn repo_root() -> PathBuf {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(|p| p.parent())
        .map(Path::to_path_buf)
        .unwrap_or_else(|| manifest.to_path_buf())
}

/// The chapter number in a folder name such as `ch07-packages`, or None.
fn chapter_number(name: &str) -> Option<u32> {
    let rest = name.strip_prefix("ch")?;
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

/// A display title such as "Ch07 · Packages", from a folder name such as `ch07-packages`.
fn chapter_title(name: &str, number: u32) -> String {
    let rest = name
        .trim_start_matches("ch")
        .trim_start_matches(|c: char| c.is_ascii_digit())
        .trim_start_matches('-');
    let words: Vec<String> = rest
        .split('-')
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect();
    format!("Ch{number:02} · {}", words.join(" "))
}

/// Every chapter folder in the repository, in chapter order. Each step is a
/// folder directly inside its chapter folder.
pub fn discover() -> Vec<Chapter> {
    let root = repo_root();
    let Ok(entries) = fs::read_dir(&root) else {
        return Vec::new();
    };

    let mut chapters: Vec<(u32, Chapter)> = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let Some(number) = chapter_number(&name) else {
            continue;
        };
        let steps = steps_in(&path);
        let homework = steps_in(&path.join("homework"));
        let sections = sections_in(&path, number);
        chapters.push((
            number,
            Chapter {
                title: chapter_title(&name, number),
                name,
                steps,
                homework,
                homework_path: path.join("homework").to_string_lossy().into_owned(),
                path: path.to_string_lossy().into_owned(),
                sections,
            },
        ));
    }

    chapters.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.name.cmp(&b.1.name)));
    chapters.into_iter().map(|(_, chapter)| chapter).collect()
}

/// A step title from a folder name such as `02-hello-world`: "Hello world".
fn step_title(name: &str) -> String {
    let rest = name.trim_start_matches(|c: char| c.is_ascii_digit()).trim_start_matches('-');
    let words: Vec<String> = rest.split('-').filter(|w| !w.is_empty()).map(str::to_string).collect();
    let joined = words.join(" ");
    let mut chars = joined.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// The number of a sub-chapter folder such as `s01-what-is-ownership`, or None.
fn section_number(name: &str) -> Option<u32> {
    let rest = name.strip_prefix('s')?;
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

/// Sub-chapters are folders named `sNN-...` inside a chapter, shown as "4.1 What is ownership".
fn sections_in(chapter: &Path, chapter_number: u32) -> Vec<Section> {
    let Ok(entries) = fs::read_dir(chapter) else {
        return Vec::new();
    };

    let mut sections: Vec<(u32, Section)> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .filter_map(|path| {
            let name = path.file_name()?.to_string_lossy().into_owned();
            let number = section_number(&name)?;
            let title = format!("{chapter_number}.{number} {}", step_title(name.strip_prefix('s').unwrap_or(&name)));
            Some((number, Section { name, title, path: path.to_string_lossy().into_owned(), steps: steps_in(&path) }))
        })
        .collect();

    sections.sort_by(|a, b| a.0.cmp(&b.0));
    sections.into_iter().map(|(_, section)| section).collect()
}

fn steps_in(practice: &Path) -> Vec<Step> {
    let Ok(entries) = fs::read_dir(practice) else {
        return Vec::new();
    };

    let mut steps: Vec<Step> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir() && (path.join("lesson.md").is_file() || path.join("Cargo.toml").is_file()))
        .map(|path| {
            let name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
            Step {
                title: step_title(&name),
                name,
                has_code: path.join("Cargo.toml").is_file(),
                path: path.to_string_lossy().into_owned(),
            }
        })
        .collect();

    steps.sort_by(|a, b| a.name.cmp(&b.name));
    steps
}
