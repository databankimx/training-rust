use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Clone)]
pub struct Step {
    pub name: String,
    pub title: String,
    pub path: String,
    pub has_code: bool,
    pub has_tests: bool,
    pub has_main: bool,
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

/// Words that are always written in capitals in titles, whatever the folder name says. Matching ignores case, so `io` shows as IO.
/// Add an acronym here when a folder name would otherwise show it as a normal word.
const ALL_CAPS_WORDS: &[&str] = &["api", "c", "cli", "ffi", "http", "io", "json", "oo", "oop", "sql", "ui", "url", "xml"];

/// The all-caps form of a word, if it's on the exceptions list.
fn all_caps(word: &str) -> Option<String> {
    ALL_CAPS_WORDS
        .contains(&word.to_lowercase().as_str())
        .then(|| word.to_uppercase())
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
            all_caps(word).unwrap_or_else(|| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
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

/// A step title from a folder name such as `02-hello-world`: "Hello world". Only the first word is capitalized,
/// except for words on the all-caps list, which are capitalized wherever they appear.
fn step_title(name: &str) -> String {
    let rest = name.trim_start_matches(|c: char| c.is_ascii_digit()).trim_start_matches('-');
    let words: Vec<String> = rest
        .split('-')
        .filter(|w| !w.is_empty())
        .enumerate()
        .map(|(index, word)| {
            if let Some(caps) = all_caps(word) {
                return caps;
            }
            if index > 0 {
                return word.to_string();
            }
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect();
    words.join(" ")
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
        .filter(|path| {
            // Sub-chapter folders (sNN-...) have overviews too, but they are listed as sections, not steps.
            // The homework folder has an overview as well, and is listed under Homework.
            let name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
            section_number(&name).is_none() && name != "homework"
        })
        .map(|path| {
            let name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
            let (has_tests, has_main) = scan_sources(&path);
            Step {
                title: step_title(&name),
                name,
                has_code: path.join("Cargo.toml").is_file(),
                has_tests,
                has_main,
                path: path.to_string_lossy().into_owned(),
            }
        })
        .collect();

    steps.sort_by(|a, b| a.name.cmp(&b.name));
    steps
}

/// Whether a step's Rust sources contain a test, and whether they contain a main function. Both are read from the files on disk.
fn scan_sources(step: &Path) -> (bool, bool) {
    let mut tests = false;
    let mut main = false;
    let mut pending = vec![step.join("src"), step.join("tests")];
    while let Some(dir) = pending.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                pending.push(p);
            } else if p.extension().is_some_and(|e| e == "rs") {
                if let Ok(text) = fs::read_to_string(&p) {
                    tests |= text.contains("#[test]");
                    main |= text.contains("fn main(");
                }
            }
        }
    }
    (tests, main)
}
