//! Code block component with syntax highlighting and copy functionality

use web_sys::window;
use yew::prelude::*;

/// Properties for the CodeBlock component
#[derive(Properties, PartialEq, Clone)]
pub struct CodeBlockProps {
    /// The code to display
    pub code: AttrValue,
    /// The language for syntax highlighting
    #[prop_or("rust".into())]
    pub language: AttrValue,
    /// Optional title for the code block
    #[prop_or_default]
    pub title: Option<AttrValue>,
    /// Whether to show line numbers
    #[prop_or(true)]
    pub show_line_numbers: bool,
}

/// Code block component with syntax highlighting and copy button
#[function_component(CodeBlock)]
pub fn code_block(props: &CodeBlockProps) -> Html {
    let copied = use_state(|| false);

    let onclick_copy = {
        let code = props.code.clone();
        let copied = copied.clone();

        Callback::from(move |_: MouseEvent| {
            let code = code.clone();
            let copied = copied.clone();

            if let Some(window) = window() {
                let clipboard = window.navigator().clipboard();
                let promise = clipboard.write_text(&code);
                let copied_inner = copied.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                    copied_inner.set(true);

                    // Reset after 2 seconds
                    gloo::timers::callback::Timeout::new(2000, move || {
                        copied.set(false);
                    })
                    .forget();
                });
            }
        })
    };

    let highlighted = highlight_rust(&props.code);

    html! {
        <div class="code-block">
            if let Some(title) = &props.title {
                <div class="code-block-header">
                    <span class="code-block-title">{ title }</span>
                    <span class="code-block-language">{ &props.language }</span>
                </div>
            }
            <div class="code-block-container">
                <button
                    class="code-block-copy"
                    onclick={onclick_copy}
                    title="Copy to clipboard"
                >
                    if *copied {
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"></polyline>
                        </svg>
                    } else {
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                            <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                        </svg>
                    }
                </button>
                <pre class={classes!("code-block-pre", props.show_line_numbers.then_some("with-line-numbers"))}>
                    <code class={format!("language-{}", props.language)}>
                        { Html::from_html_unchecked(AttrValue::from(highlighted)) }
                    </code>
                </pre>
            </div>
        </div>
    }
}

/// Simple Rust syntax highlighting
fn highlight_rust(code: &str) -> String {
    let keywords = [
        "use", "fn", "let", "mut", "pub", "struct", "enum", "impl", "trait",
        "where", "for", "if", "else", "match", "return", "const", "static",
        "mod", "crate", "self", "super", "as", "in", "move", "ref", "type",
        "dyn", "async", "await", "loop", "while", "break", "continue",
    ];

    let types = [
        "Html", "String", "Vec", "Option", "Result", "bool", "u8", "u16",
        "u32", "u64", "i8", "i16", "i32", "i64", "f32", "f64", "usize",
        "isize", "str", "Self", "Callback", "AttrValue", "Properties",
    ];

    let macros = ["html!", "vec!", "format!", "println!", "panic!", "assert!"];

    let mut result = String::new();
    let mut chars = code.chars().peekable();
    let mut in_string = false;
    let mut string_char = '"';

    while let Some(c) = chars.next() {
        // Handle comments
        if !in_string && c == '/' {
            if chars.peek() == Some(&'/') {
                // Line comment
                result.push_str("<span class=\"hljs-comment\">/");
                while let Some(ch) = chars.next() {
                    if ch == '\n' {
                        result.push_str("</span>\n");
                        break;
                    }
                    push_escaped(&mut result, ch);
                }
                continue;
            }
        }

        // Handle strings
        if c == '"' || c == '\'' {
            if !in_string {
                in_string = true;
                string_char = c;
                result.push_str("<span class=\"hljs-string\">");
                result.push(c);
            } else if c == string_char {
                result.push(c);
                result.push_str("</span>");
                in_string = false;
            } else {
                result.push(c);
            }
            continue;
        }

        if in_string {
            push_escaped(&mut result, c);
            continue;
        }

        // Handle identifiers and keywords
        if c.is_alphabetic() || c == '_' {
            let mut word = String::new();
            word.push(c);
            while let Some(&next) = chars.peek() {
                if next.is_alphanumeric() || next == '_' || next == '!' {
                    word.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            if macros.iter().any(|m| *m == word) {
                result.push_str(&format!("<span class=\"hljs-keyword\">{}</span>", escape_html(&word)));
            } else if keywords.iter().any(|k| *k == word) {
                result.push_str(&format!("<span class=\"hljs-keyword\">{}</span>", escape_html(&word)));
            } else if types.iter().any(|t| *t == word) {
                result.push_str(&format!("<span class=\"hljs-type\">{}</span>", escape_html(&word)));
            } else if word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                result.push_str(&format!("<span class=\"hljs-type\">{}</span>", escape_html(&word)));
            } else {
                result.push_str(&escape_html(&word));
            }
            continue;
        }

        // Handle numbers
        if c.is_numeric() {
            let mut num = String::new();
            num.push(c);
            while let Some(&next) = chars.peek() {
                if next.is_alphanumeric() || next == '.' || next == '_' {
                    num.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            result.push_str(&format!("<span class=\"hljs-number\">{}</span>", escape_html(&num)));
            continue;
        }

        // Handle attributes
        if c == '#' && chars.peek() == Some(&'[') {
            result.push_str("<span class=\"hljs-meta\">#");
            let mut depth = 0;
            while let Some(ch) = chars.next() {
                push_escaped(&mut result, ch);
                if ch == '[' {
                    depth += 1;
                } else if ch == ']' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
            }
            result.push_str("</span>");
            continue;
        }

        push_escaped(&mut result, c);
    }

    result
}

fn escape_html(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '&' => result.push_str("&amp;"),
            '"' => result.push_str("&quot;"),
            _ => result.push(c),
        }
    }
    result
}

fn push_escaped(result: &mut String, c: char) {
    match c {
        '<' => result.push_str("&lt;"),
        '>' => result.push_str("&gt;"),
        '&' => result.push_str("&amp;"),
        '"' => result.push_str("&quot;"),
        _ => result.push(c),
    }
}
