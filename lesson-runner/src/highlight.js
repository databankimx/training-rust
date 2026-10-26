// A small Rust tokenizer for the Source and Lesson panes. It is not a parser:
// it recognizes comments, strings, numbers, keywords, and identifiers well
// enough to colour code the way Visual Studio does.

const KEYWORDS = new Set([
  "as", "async", "await", "const", "crate", "dyn", "enum", "extern", "false",
  "fn", "impl", "let", "mod", "move", "mut", "pub", "ref", "self", "Self",
  "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
]);

const CONTROL = new Set([
  "break", "continue", "else", "for", "if", "in", "loop", "match", "return", "while",
]);

const TYPES = new Set([
  "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize",
  "u8", "u16", "u32", "u64", "u128", "usize", "str", "String", "Vec", "Option",
  "Result", "Box", "Some", "None", "Ok", "Err",
]);

// Order matters: the first alternative that matches at a position wins.
const TOKEN = new RegExp(
  [
    String.raw`\/\*[\s\S]*?\*\/`,                       // block comment
    String.raw`\/\/[^\n]*`,                             // line comment
    String.raw`r#*"[\s\S]*?"#*`,                        // raw string
    String.raw`"(?:\\[\s\S]|[^"\\])*"`,                 // string
    String.raw`'(?:\\u\{[0-9a-fA-F_]+\}|\\.|[^'\\\n])'`, // char literal
    String.raw`'[A-Za-z_][A-Za-z0-9_]*`,                // lifetime
    String.raw`#!?\[[^\]\n]*\]`,                        // attribute
    String.raw`\b(?:0[xob][0-9a-fA-F_]+|\d[\d_]*(?:\.\d[\d_]*)?(?:[eE][+-]?\d+)?)(?:[iuf](?:8|16|32|64|128|size))?\b`, // number
    String.raw`[A-Za-z_][A-Za-z0-9_]*!?`,               // identifier or macro name
    String.raw`\s+`,                                    // whitespace
    String.raw`[\s\S]`,                                 // anything else
  ].join("|"),
  "g",
);

function escapeHtml(text) {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

function classifyWord(word, previous, next) {
  if (word.endsWith("!")) return "macro";
  if (KEYWORDS.has(word)) return "keyword";
  if (CONTROL.has(word)) return "control";
  if (TYPES.has(word)) return "type";
  if (previous === "fn") return "function";
  if (next === "(") return /^[A-Z]/.test(word) ? "type" : "function";
  if (/^[A-Z][A-Z0-9_]+$/.test(word)) return "constant";
  if (/^[A-Z]/.test(word)) return "type";
  return "variable";
}

function classify(text, source, end, previous) {
  if (text.startsWith("/*") || text.startsWith("//")) return "comment";
  if (text.startsWith('"') || text.startsWith("r")) {
    if (/^r#*"/.test(text) || text.startsWith('"')) return "string";
  }
  if (text.startsWith("'")) {
    return text.length >= 3 && text.endsWith("'") ? "string" : "keyword";
  }
  if (text.startsWith("#")) return "attribute";
  if (/^\d/.test(text)) return "number";
  if (/^[A-Za-z_]/.test(text)) {
    const next = (source.slice(end).match(/^\s*(.)/) || [])[1];
    return classifyWord(text, previous, next);
  }
  return "";
}

const PY_KEYWORDS = new Set([
  "and", "as", "assert", "async", "await", "class", "def", "del", "from", "global", "import",
  "in", "is", "lambda", "nonlocal", "not", "or", "with", "yield",
]);

const PY_CONTROL = new Set([
  "break", "continue", "elif", "else", "except", "finally", "for", "if", "pass", "raise", "return",
  "try", "while",
]);

const PY_CONSTANTS = new Set(["True", "False", "None"]);

const PY_BUILTINS = new Set([
  "bool", "dict", "float", "int", "list", "len", "open", "print", "range", "set", "str", "tuple",
  "super", "type",
]);

const PY_TOKEN = new RegExp(
  [
    String.raw`#[^\n]*`,                                  // comment
    String.raw`"""[\s\S]*?"""|'''[\s\S]*?'''`,         // triple-quoted string
    String.raw`"(?:\\[\s\S]|[^"\\\n])*"|'(?:\\[\s\S]|[^'\\\n])*'`, // string
    String.raw`\b\d[\d_]*(?:\.\d+)?\b`,                  // number
    String.raw`[A-Za-z_][A-Za-z0-9_]*`,                  // identifier
    String.raw`\s+`,                                     // whitespace
    String.raw`[\s\S]`,                                  // anything else
  ].join("|"),
  "g",
);

function classifyPython(text, source, end) {
  if (text.startsWith("#")) return "comment";
  if (text.startsWith('"') || text.startsWith("'")) return "string";
  if (/^\d/.test(text)) return "number";
  if (/^[A-Za-z_]/.test(text)) {
    if (PY_CONSTANTS.has(text)) return "constant";
    if (PY_KEYWORDS.has(text)) return "keyword";
    if (PY_CONTROL.has(text)) return "control";
    if (PY_BUILTINS.has(text)) return "type";
    const next = (source.slice(end).match(/^\s*(.)/) || [])[1];
    if (next === "(") return "function";
    return "variable";
  }
  return "";
}

/** Returns Python source as HTML with `tok-*` spans, coloured like the Rust highlighting. */
export function highlightPython(source) {
  const parts = [];
  PY_TOKEN.lastIndex = 0;
  let match;
  while ((match = PY_TOKEN.exec(source)) !== null) {
    const text = match[0];
    const type = classifyPython(text, source, PY_TOKEN.lastIndex);
    parts.push(type ? `<span class="tok-${type}">${escapeHtml(text)}</span>` : escapeHtml(text));
  }
  return parts.join("");
}

/** Returns the source as HTML with `tok-*` spans. */
export function highlightRust(source) {
  const parts = [];
  let previous = "";
  TOKEN.lastIndex = 0;
  let match;
  while ((match = TOKEN.exec(source)) !== null) {
    const text = match[0];
    const type = classify(text, source, TOKEN.lastIndex, previous);
    if (!/^\s+$/.test(text)) previous = text;
    parts.push(type ? `<span class="tok-${type}">${escapeHtml(text)}</span>` : escapeHtml(text));
  }
  return parts.join("");
}
