import { highlightRust } from "./highlight.js";
import { icon } from "./icons.js";
import { installExternalLinks } from "./links.js";
import { renderLessonInto } from "./lesson-render.js";
import { blockPageZoom, enableZoom } from "./zoom.js";

const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const chaptersEl = document.querySelector("#chapters");
const stepsEl = document.querySelector("#steps");
const titleEl = document.querySelector("#title");
const subtitleEl = document.querySelector("#subtitle");
const output = document.querySelector("#output");
const runButton = document.querySelector("#run");
const testButton = document.querySelector("#test");
const stopButton = document.querySelector("#stop");
const clearButton = document.querySelector("#clear");
const inputForm = document.querySelector("#input-form");
const input = document.querySelector("#input");
const sendButton = document.querySelector("#send");
const themeToggle = document.querySelector("#theme");
const sourceView = document.querySelector("#source-view");
const lessonEl = document.querySelector("#lesson");
const editor = document.querySelector("#editor");
const gutter = document.querySelector("#gutter");
const highlightEl = document.querySelector("#highlight");
const modifiedEl = document.querySelector("#modified");
const resetButton = document.querySelector("#reset");
const spoilerEl = document.querySelector("#spoiler");
const revealButton = document.querySelector("#reveal");
const copyOutputButton = document.querySelector("#copy-output");
const copySourceButton = document.querySelector("#copy-source");
const popoutButton = document.querySelector("#popout");

// Source files: a step can have several .rs files. The drop-down above the editor picks one, and main.rs is the default.
const fileSelect = document.createElement("select");
fileSelect.id = "file-select";
fileSelect.title = "Choose a source file in this step";
fileSelect.style.margin = "4px 8px";
sourceView.before(fileSelect);
let currentFile = null;
let originalMain = "";
let mainDraft = null;
let fileCount = 0;

// Quick picks: whole commands from the step's commands.txt. Choosing one fills the argument box.
const commandPick = document.createElement("select");
commandPick.className = "command-pick hidden";
commandPick.title = "Quick picks for this step";
input.before(commandPick);

// The drop-down is shown only on the Source tab, for steps with more than one source file.
function updateFileSelect() {
  fileSelect.classList.toggle("hidden", activeTab !== "source" || spoilerHidden || fileCount < 2);
}

// Backtrace toggle: when it is on, each run sets RUST_BACKTRACE, so a panic shows where it came from.
const BACKTRACE_KEY = "rust-training-backtrace";
const backtraceLabel = document.createElement("label");
backtraceLabel.title = "Show a backtrace when a program panics";
backtraceLabel.style.fontSize = "12px";
backtraceLabel.style.marginLeft = "32px";
backtraceLabel.style.fontWeight = "normal";
backtraceLabel.style.letterSpacing = "normal";
const backtraceToggle = document.createElement("input");
backtraceToggle.type = "checkbox";
backtraceToggle.checked = localStorage.getItem(BACKTRACE_KEY) === "1";
backtraceToggle.addEventListener("change", () => {
  localStorage.setItem(BACKTRACE_KEY, backtraceToggle.checked ? "1" : "0");
});
backtraceLabel.append(backtraceToggle, " Backtrace");
// The checkbox sits to the right of the OUTPUT heading, in a group that keeps the two together on the left of the bar.
const paneLabel = document.querySelector(".pane-label");
const paneLeft = document.createElement("span");
paneLeft.className = "pane-left";
paneLabel.before(paneLeft);
paneLeft.append(paneLabel, backtraceLabel);

let chapters = [];
let selectedChapter = null;
let selectedStep = null;
let homeworkMode = false;
let selectedSection = null;
let running = false;
let original = "";
let lessonText = "";
let popped = false;
let spoilerHidden = false;

// Output

/** Colours a line by what it says, so compiler status is not mistaken for an error. */
function outputClass(line, stream) {
  if (/^\s*(Compiling|Checking|Building|Finished|Running|Fresh|Updating|Locking|Downloading|Adding)\b/.test(line)) {
    return "status";
  }
  if (/^\s*error(\[|:)/.test(line)) return "error";
  if (/^\s*warning(\[|:)/.test(line)) return "warning";
  return stream === "stderr" ? "stderr" : "";
}

function appendOutput(text, stream) {
  for (const line of text.split(/(?<=\n)/)) {
    const span = document.createElement("span");
    span.className = outputClass(line, stream);
    span.textContent = line;
    output.appendChild(span);
  }
  output.scrollTop = output.scrollHeight;
}

function appendNote(text, className) {
  const span = document.createElement("span");
  span.className = className;
  span.textContent = text;
  output.appendChild(span);
  output.scrollTop = output.scrollHeight;
}

// Buttons and chapters

function updateButtons() {
  const hasTests = Boolean(selectedStep && selectedStep.has_tests);
  const testOnly = hasTests && !selectedStep.has_main;
  // A step whose only program is its tests shows Test in place of Run. Otherwise the two sit side by side.
  runButton.classList.toggle("hidden", testOnly);
  testButton.classList.toggle("hidden", !hasTests);
  const ready = !running && Boolean(selectedStep) && selectedStep.has_code && !spoilerHidden;
  runButton.disabled = !ready;
  testButton.disabled = !ready;
  stopButton.disabled = !running;
  // While nothing is running, the input box takes the arguments for a run or a test.
  input.disabled = !running && !ready;
  sendButton.disabled = !running && !ready;
  input.placeholder = running ? "type a line for the program, then Enter" : "arguments, then Enter to run";
  sendButton.textContent = running ? "Send" : "Go";
  commandPick.classList.toggle("hidden", running || !ready || commandPick.options.length <= 1);
}

function renderChapters() {
  chaptersEl.replaceChildren();
  for (const chapter of chapters) {
    const row = document.createElement("div");
    row.className = "chapter";
    if (chapter === selectedChapter) row.classList.add("open");
    if (chapter === selectedChapter && !homeworkMode && !selectedSection) row.classList.add("selected");

    const chevron = document.createElement("span");
    chevron.className = "chevron";
    chevron.textContent = "\u25B8";
    const label = document.createElement("span");
    label.textContent = chapter.title;

    row.append(chevron, label);
    row.addEventListener("click", () => selectChapter(chapter));
    chaptersEl.appendChild(row);

    // Sub-chapters appear under the selected chapter, collapsed until one is chosen.
    if (chapter === selectedChapter) {
      for (const section of chapter.sections) {
        const sectionRow = document.createElement("div");
        sectionRow.className = "section-row";
        if (section === selectedSection) sectionRow.classList.add("open", "selected");
        const sectionChevron = document.createElement("span");
        sectionChevron.className = "chevron";
        sectionChevron.textContent = "\u25B8";
        const sectionLabel = document.createElement("span");
        sectionLabel.textContent = section.title;
        sectionRow.append(sectionChevron, sectionLabel);
        sectionRow.addEventListener("click", (event) => {
          event.stopPropagation();
          selectSection(chapter, section);
        });
        chaptersEl.appendChild(sectionRow);
      }
    }

    // Homework appears only under the selected chapter, collapsed until it is chosen.
    if (chapter === selectedChapter && chapter.homework.length) {
      const homework = document.createElement("div");
      homework.className = "homework-row";
      if (homeworkMode) homework.classList.add("open", "selected");
      const homeworkChevron = document.createElement("span");
      homeworkChevron.className = "chevron";
      homeworkChevron.textContent = "\u25B8";
      const homeworkLabel = document.createElement("span");
      homeworkLabel.textContent = "Homework";
      homework.append(homeworkChevron, homeworkLabel);
      homework.addEventListener("click", (event) => {
        event.stopPropagation();
        selectHomework(chapter);
      });
      chaptersEl.appendChild(homework);
    }
  }
}

function renderSteps() {
  stepsEl.replaceChildren();
  document.querySelector(".steps-heading").textContent = homeworkMode ? "Homework" : "Steps";
  if (!selectedChapter) return;
  const items = homeworkMode ? selectedChapter.homework : selectedSection ? selectedSection.steps : selectedChapter.steps;
  for (const step of items) {
    const button = document.createElement("button");
    button.className = "step";
    if (step === selectedStep) button.classList.add("selected");
    button.textContent = step.title;
    button.addEventListener("click", () => selectStep(selectedChapter, step));
    stepsEl.appendChild(button);
  }
}

function selectSection(chapter, section) {
  selectedChapter = chapter;
  selectedSection = section;
  homeworkMode = false;
  renderChapters();
  renderSteps();
  showOverview(section.path, "section:" + section.path, section.title, chapter.title);
}

async function selectHomework(chapter) {
  selectedChapter = chapter;
  homeworkMode = true;
  selectedSection = null;
  output.replaceChildren();
  selectedStep = null;
  spoilerHidden = false;
  titleEl.textContent = "Homework";
  subtitleEl.textContent = chapter.title;
  renderChapters();
  renderSteps();
  updateButtons();
  setTab("lesson");
  rememberLessonScroll();
  lessonText = await invoke("read_lesson", { path: chapter.homework_path });
  renderLesson();
  showLessonAt("homework:" + chapter.homework_path);
  invoke("set_lesson", { title: `${chapter.title} · Homework`, text: lessonText });
}

// Selecting a chapter or section shows its overview lesson, if it has one.
async function showOverview(path, key, title, subtitle) {
  selectedStep = null;
  spoilerHidden = false;
  titleEl.textContent = title;
  subtitleEl.textContent = subtitle;
  updateButtons();
  setTab("lesson");
  rememberLessonScroll();
  lessonText = await invoke("read_lesson", { path });
  if (!lessonText.trim()) lessonText = `${title} has no overview yet.`;
  renderLesson();
  showLessonAt(key);
  invoke("set_lesson", { title, text: lessonText });
}

function selectChapter(chapter) {
  selectedChapter = chapter;
  selectedSection = null;
  homeworkMode = false;
  renderChapters();
  renderSteps();
  showOverview(chapter.path, "chapter:" + chapter.path, chapter.title, "");
}

// Links in an overview lesson, such as #goto/s02-example-structs, select the named child.
listen("goto", (event) => {
  const chapter = selectedChapter;
  if (!chapter) return;
  const name = event.payload;
  const section = chapter.sections.find((s) => s.name === name);
  if (section) {
    selectSection(chapter, section);
    return;
  }
  const step = (selectedSection ? selectedSection.steps : chapter.steps).find((s) => s.name === name);
  if (step) selectStep(chapter, step);
});

async function selectStep(chapter, step) {
  // A program that is still running belongs to the step being left, so stop it before showing the new one.
  if (running) await stop();
  selectedChapter = chapter;
  homeworkMode = chapter.homework.includes(step);
  selectedSection = chapter.sections.find((section) => section.steps.includes(step)) || null;
  if (selectedStep !== step) output.replaceChildren();
  selectedStep = step;
  titleEl.textContent = step.title;
  subtitleEl.textContent = homeworkMode ? `${chapter.title} · Homework` : selectedSection ? selectedSection.title : chapter.title;
  spoilerHidden = homeworkMode && !isRevealed(step);
  renderChapters();
  renderSteps();
  updateButtons();
  setTab(activeTab);
  input.value = "";
  loadCommandPicks(step);
  if (spoilerHidden) {
    await loadLesson(step);
  } else {
    await Promise.all([loadSource(step), loadLesson(step)]);
  }
}

// Source editor

function renderEditor() {
  const text = editor.value;
  highlightEl.innerHTML = highlightRust(text) + "\n";
  const lineCount = text.split("\n").length;
  gutter.textContent = Array.from({ length: lineCount }, (_, i) => i + 1).join("\n");
  const modified = text !== original;
  modifiedEl.classList.toggle("hidden", !modified);
  resetButton.disabled = !modified;
}

function syncScroll() {
  highlightEl.scrollTop = editor.scrollTop;
  highlightEl.scrollLeft = editor.scrollLeft;
  gutter.scrollTop = editor.scrollTop;
}

async function loadSource(step) {
  mainDraft = null;
  originalMain = "";
  currentFile = null;
  if (!step.has_code) {
    fileSelect.replaceChildren();
    fileCount = 0;
    updateFileSelect();
    editor.value = "// This step has no code to run. See the Lesson tab.";
    original = editor.value;
    editor.readOnly = true;
    renderEditor();
    return;
  }
  try {
    const files = await invoke("list_source_files", { path: step.path });
    fileCount = files.length;
    updateFileSelect();
    fileSelect.replaceChildren(
      ...files.map((file) => {
        const option = document.createElement("option");
        option.value = file;
        option.textContent = file;
        return option;
      }),
    );
    await loadSourceFile(step, "main.rs");
  } catch (error) {
    original = "";
    editor.value = `${error}`;
    renderEditor();
  }
}

/** Shows one source file. Only main.rs can be edited, because only main.rs is sent to the runner. */
async function loadSourceFile(step, file) {
  if (currentFile === "main.rs" && !editor.readOnly) mainDraft = editor.value;
  currentFile = file;
  fileSelect.value = file;
  const text = await invoke("read_source", { path: step.path, file });
  if (file === "main.rs") {
    originalMain = text;
    editor.value = mainDraft ?? text;
    editor.readOnly = false;
  } else {
    editor.value = text;
    editor.readOnly = true;
  }
  original = text;
  editor.scrollTop = 0;
  renderEditor();
}

fileSelect.addEventListener("change", () => {
  if (selectedStep) loadSourceFile(selectedStep, fileSelect.value);
});

function resetEditor() {
  editor.value = original;
  renderEditor();
}

editor.addEventListener("input", renderEditor);
editor.addEventListener("scroll", syncScroll);
editor.addEventListener("keydown", (event) => {
  if (event.key === "Tab") {
    event.preventDefault();
    const start = editor.selectionStart;
    editor.setRangeText("    ", start, editor.selectionEnd, "end");
    renderEditor();
  }
});
resetButton.addEventListener("click", resetEditor);

// Lesson (Markdown)

const REVEALED_PREFIX = "rust-training-revealed:";

function isRevealed(step) {
  return localStorage.getItem(REVEALED_PREFIX + step.path) === "1";
}

async function loadLesson(step) {
  rememberLessonScroll();
  lessonText = await invoke("read_lesson", { path: step.path });
  // Once a homework example has been revealed, its walkthrough is shown with the exercise.
  if (homeworkMode && isRevealed(step)) {
    const solution = await invoke("read_step_file", { path: step.path, file: "solution.md" });
    if (solution.trim()) lessonText += "\n\n---\n\n" + solution;
  }
  renderLesson();
  showLessonAt(step.path);
  invoke("set_lesson", { title: step.title, text: lessonText });
}

function renderLesson() {
  if (popped) {
    const note = document.createElement("p");
    note.textContent = "This lesson is open in its own window.";
    const dock = document.createElement("button");
    dock.innerHTML = `${icon("dock")} Dock lesson here`;
    dock.addEventListener("click", () => invoke("dock_lesson"));
    lessonEl.replaceChildren(note, dock);
    return;
  }
  if (!lessonText.trim()) {
    lessonEl.textContent = "No lesson text yet for this step.";
    return;
  }
  lessonEl.innerHTML = "";
  renderLessonInto(lessonEl, lessonText);
}

// Quick picks for the step. The text after cargo test or cargo run becomes the argument box's contents.
async function loadCommandPicks(step) {
  commandPick.replaceChildren();
  const placeholder = document.createElement("option");
  placeholder.textContent = "Quick picks";
  placeholder.value = "";
  commandPick.appendChild(placeholder);
  if (step.has_code) {
    const commands = await invoke("read_commands", { path: step.path });
    for (const command of commands) {
      const option = document.createElement("option");
      option.textContent = command;
      option.value = command.replace(/^cargo\s+(test|run)\s*/, "");
      commandPick.appendChild(option);
    }
  }
  updateButtons();
}

commandPick.addEventListener("change", () => {
  // Always replace the box's text, so choosing "cargo test" (no arguments) clears the previous arguments.
  input.value = commandPick.value;
  input.focus();
});

// Running

async function run(mode, args = []) {
  if (!selectedStep || running) return;
  // With no mode given (F5), a test-only step runs its tests, and any other step runs its program.
  const kind = mode ?? (selectedStep.has_tests && !selectedStep.has_main ? "test" : "run");
  running = true;
  output.replaceChildren();
  updateButtons();
  // Send main.rs only when its text differs from the file on disk. Other files are read-only.
  const mainText = currentFile === "main.rs" ? editor.value : mainDraft ?? originalMain;
  const edited = mainText !== originalMain ? mainText : null;
  try {
    await invoke("run_step", { path: selectedStep.path, source: edited, backtrace: backtraceToggle.checked, test: kind === "test", args });
  } catch (error) {
    appendOutput(`${error}\n`, "stderr");
    running = false;
    updateButtons();
  }
}

async function stop() {
  if (!running) return;
  await invoke("stop_run");
}

async function sendInput(event) {
  event.preventDefault();
  // While a program is running, the box sends a line to it. Otherwise it gives the arguments for a run.
  if (!running) {
    if (!selectedStep) return;
    const args = input.value.trim().split(/\s+/).filter(Boolean);
    run(selectedStep.has_tests ? "test" : "run", args);
    return;
  }
  const text = input.value;
  input.value = "";
  appendNote(`${text}\n`, "input");
  await invoke("send_input", { text: `${text}\n` });
}

// Each lesson remembers where the reader left it. A lesson opened for the first time starts at the top.
const lessonScroll = new Map();
let currentLessonKey = null;
let pendingLessonScroll = 0;

function rememberLessonScroll() {
  if (currentLessonKey !== null && !lessonEl.classList.contains("hidden")) {
    lessonScroll.set(currentLessonKey, lessonEl.scrollTop);
  }
}

function applyLessonScroll() {
  // Set the offset now, then again once the new content has laid out, and again shortly
  // after, since diagrams and quizzes can change the height of the lesson as they render.
  const offset = pendingLessonScroll;
  lessonEl.scrollTop = offset;
  requestAnimationFrame(() => {
    lessonEl.scrollTop = offset;
  });
  setTimeout(() => {
    lessonEl.scrollTop = offset;
  }, 150);
}

function showLessonAt(key) {
  currentLessonKey = key;
  pendingLessonScroll = lessonScroll.get(key) ?? 0;
  applyLessonScroll();
}

let activeTab = "source";
function setTab(name) {
  if (activeTab === "lesson" && name !== "lesson") rememberLessonScroll();
  activeTab = name;
  document.querySelectorAll(".tab").forEach((tab) => {
    tab.classList.toggle("active", tab.dataset.tab === name);
  });
  const showSource = name === "source" && !spoilerHidden;
  sourceView.classList.toggle("hidden", !showSource);
  spoilerEl.classList.toggle("hidden", !(name === "source" && spoilerHidden));
  lessonEl.classList.toggle("hidden", name !== "lesson");
  document.querySelector("#reset").classList.toggle("hidden", !showSource);
  modifiedEl.classList.toggle("hidden", !showSource || editor.value === original);
  updateFileSelect();
  if (name === "lesson") applyLessonScroll();
}

runButton.addEventListener("click", () => run("run"));
testButton.addEventListener("click", () => run("test"));

// A short message that fades out. The text is shown for a moment, then removed.
function showToast(message) {
  const toast = document.createElement("div");
  toast.className = "toast";
  toast.textContent = message;
  document.body.append(toast);
  setTimeout(() => toast.classList.add("fade"), 400);
  setTimeout(() => toast.remove(), 1250);
}

// Undocumented: Ctrl+Shift+Alt+Backspace clears quiz answers and revealed solutions. Backtrace and theme settings are kept.
// It listens in the capture phase, so the editor and other controls cannot swallow the key before it arrives.
window.addEventListener(
  "keydown",
  (event) => {
    if (!(event.ctrlKey && event.shiftKey && event.altKey && event.key === "Backspace")) return;
    event.preventDefault();
    for (const key of Object.keys(localStorage)) {
      if (key.startsWith("rust-training-quiz:") || key.startsWith("rust-training-revealed:")) {
        localStorage.removeItem(key);
      }
    }
    if (selectedStep) selectStep(selectedChapter, selectedStep);
    showToast("Progress reset");
  },
  true,
);

// Spoiler: reveals the example solution for a homework step, and remembers that it was revealed.
revealButton.addEventListener("click", async () => {
  if (!selectedStep) return;
  localStorage.setItem(REVEALED_PREFIX + selectedStep.path, "1");
  spoilerHidden = false;
  updateButtons();
  await Promise.all([loadSource(selectedStep), loadLesson(selectedStep)]);
  setTab("source");
});

// Copy and pop-out

function copyText(text) {
  navigator.clipboard.writeText(text).catch((error) => appendNote(`${error}\n`, "stderr"));
}

copyOutputButton.addEventListener("click", () => copyText(output.textContent));
copySourceButton.addEventListener("click", () => {
  const onLesson = document.querySelector(".tab.active").dataset.tab === "lesson";
  copyText(onLesson ? lessonText : editor.value);
});
popoutButton.addEventListener("click", () => {
  invoke("open_lesson_window").catch((error) => appendNote(`${error}\n`, "stderr"));
});
installExternalLinks(lessonEl);

// Ctrl + mouse wheel zooms each pane's text. The rest of the window keeps its size.
blockPageZoom();
enableZoom(output, "output", 13);
enableZoom(sourceView, "source", 13);
enableZoom(lessonEl, "lesson", 14);
stopButton.addEventListener("click", stop);
clearButton.addEventListener("click", () => output.replaceChildren());
inputForm.addEventListener("submit", sendInput);
themeToggle.addEventListener("change", () => {
  document.body.classList.toggle("light", themeToggle.checked);
  invoke("set_theme", { light: themeToggle.checked });
});
document.querySelectorAll(".tab").forEach((tab) => {
  tab.addEventListener("click", () => setTab(tab.dataset.tab));
});

document.addEventListener("keydown", (event) => {
  if (event.key === "F5") {
    event.preventDefault();
    run();
  } else if (event.key === "Escape") {
    if (running) {
      stop();
    } else if (editor.value !== original) {
      resetEditor();
    }
  }
});

listen("lesson-popped", () => {
  popped = true;
  renderLesson();
});

listen("lesson-docked", () => {
  popped = false;
  renderLesson();
});

listen("run-output", (event) => {
  appendOutput(event.payload.text, event.payload.stream);
});

listen("run-finished", (event) => {
  const code = event.payload.code;
  appendNote(`\n\u2014 Finished${code === null ? "" : ` with exit code ${code}`} \u2014\n`, "status");
  running = false;
  updateButtons();
});

// Splitters: drag to resize the sidebar, the steps list, and the source pane.

function drag(handle, onMove) {
  handle.addEventListener("pointerdown", (event) => {
    event.preventDefault();
    handle.setPointerCapture(event.pointerId);
    handle.addEventListener("pointermove", onMove);
    const stopDrag = () => {
      handle.removeEventListener("pointermove", onMove);
      handle.removeEventListener("pointerup", stopDrag);
    };
    handle.addEventListener("pointerup", stopDrag);
  });
}

const root = document.documentElement.style;

drag(document.querySelector("#split-sidebar"), (event) => {
  const width = Math.min(Math.max(event.clientX, 220), window.innerWidth - 420);
  root.setProperty("--sidebar-width", `${width}px`);
});

drag(document.querySelector("#split-tree"), (event) => {
  const top = chaptersEl.getBoundingClientRect().top;
  const height = Math.min(Math.max(event.clientY - top, 80), window.innerHeight - top - 200);
  root.setProperty("--chapters-height", `${height}px`);
});

drag(document.querySelector("#split-source"), (event) => {
  const height = Math.min(Math.max(window.innerHeight - event.clientY, 140), window.innerHeight - 220);
  root.setProperty("--source-height", `${height}px`);
});

invoke("list_chapters")
  .then((result) => {
    chapters = result;
    renderChapters();
  })
  .catch((error) => appendOutput(`${error}\n`, "stderr"));

copyOutputButton.innerHTML = icon("copy");
copySourceButton.innerHTML = icon("copy");
popoutButton.innerHTML = icon("popout");
document.querySelector("#moon").innerHTML = icon("moon");
document.querySelector("#sun").innerHTML = icon("sun");

updateButtons();
setTab("source");
