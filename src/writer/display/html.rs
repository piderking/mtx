use crate::{
    ast::{Statement, module::Metadata},
    parser::markdown::{Inline, MarkdownElement},
};

pub enum Display {
    Equation(Vec<Statement>),
    Markdown(Vec<MarkdownElement>),
}

pub struct Displayed(Vec<Display>);

impl ToString for Display {
    fn to_string(&self) -> String {
        match self {
            Display::Equation(statements) => todo!(),
            Display::Markdown(markdown_elements) => markdown_elements
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }
}

impl ToString for Displayed {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl ToString for Inline {
    fn to_string(&self) -> String {
        match self {
            Inline::Text(text) => html_escape(text),
            Inline::Bold(text) => {
                format!("<strong class=\"font-bold\">{}</strong>", html_escape(text))
            }
            Inline::Italic(text) => format!("<em class=\"italic\">{}</em>", html_escape(text)),
            Inline::Code(text) => format!(
                "<code class=\"px-1.5 py-0.5 bg-gray-800 text-sm rounded font-mono text-pink-400\">{}</code>",
                html_escape(text)
            ),
            Inline::Link(text, url) => format!(
                "<a href=\"{}\" class=\"text-blue-400 hover:text-gray-100\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a>",
                html_escape(url),
                html_escape(text)
            ),
        }
    }
}

impl ToString for MarkdownElement {
    fn to_string(&self) -> String {
        match self {
            MarkdownElement::Heading(level, text) => {
                let classes = match level {
                    1 => "text-2xl font-bold mb-4 text-gray-100",
                    2 => "text-xl font-bold mt-6 mb-3 text-gray-100",
                    3 => "text-lg font-bold mt-5 mb-3 text-gray-100",
                    4 => "text-base font-bold mt-4 mb-2 text-gray-100",
                    5 => "text-sm font-bold mt-3 mb-2 text-gray-100",
                    _ => "text-sm font-bold mt-2 mb-2 text-gray-100",
                };
                format!(
                    "<h{} class=\"{}\">{}</h{}>",
                    level,
                    classes,
                    html_escape(text),
                    level
                )
            }
            MarkdownElement::Paragraph(inlines) => {
                let content = inlines.iter().map(|i| i.to_string()).collect::<String>();
                format!("<p class=\"mb-4 text-gray-300 leading-7\">{}</p>", content)
            }
            MarkdownElement::CodeBlock(lang, code) => {
                let lang_display = if lang.is_empty() {
                    String::new()
                } else {
                    format!(
                        "<div class=\"text-xs text-gray-400 mb-2 font-mono\">{}</div>",
                        html_escape(lang)
                    )
                };
                format!(
                    "<div class=\"mb-4 rounded overflow-hidden border border-gray-700\">{}<pre class=\"bg-gray-800 p-4 overflow-x-auto\"><code class=\"text-sm font-mono text-gray-200\">{}</code></pre></div>",
                    lang_display,
                    html_escape(code)
                )
            }
            MarkdownElement::UnorderedList(items) => {
                let items_html = items
                    .iter()
                    .map(|item| format!("<li class=\"mb-2\">{}</li>", html_escape(item)))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    "<ul class=\"list-disc list-inside mb-4 space-y-1 text-gray-300\">\n{}\n</ul>",
                    items_html
                )
            }
            MarkdownElement::OrderedList(items) => {
                let items_html = items
                    .iter()
                    .map(|item| format!("<li class=\"mb-2\">{}</li>", html_escape(item)))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    "<ol class=\"list-decimal list-inside mb-4 space-y-1 text-gray-300\">\n{}\n</ol>",
                    items_html
                )
            }
            MarkdownElement::Blockquote(text) => {
                format!(
                    "<blockquote class=\"border-l-4 border-blue-500 pl-4 py-2 mb-4 italic text-gray-400 bg-gray-800/50\">{}</blockquote>",
                    html_escape(text)
                )
            }
            MarkdownElement::HorizontalRule => {
                "<hr class=\"my-8 border-t border-gray-700\" />".to_string()
            }
            MarkdownElement::LineBreak => "<div class=\"mb-4\"></div>".to_string(),
        }
    }
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use nom::Parser;

    use crate::parser::markdown::parse_markdown;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_display_markdown_to_html() {
        let markdown = r#"# Title

This is a **paragraph** with `code`.

- List item 1
- List item 2

```rust
fn main() {}
```
"#;

        let (s, result) = parse_markdown(markdown).expect("parse should work");
        println!("{} ==> {:?}", s, result);
        println!("{}", Display::Markdown(result).to_string())
    }
}
