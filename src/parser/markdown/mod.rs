use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{char, digit1, line_ending, multispace0, not_line_ending, space0},
    combinator::{map, opt, recognize},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
};

#[derive(Debug, PartialEq, Clone)]
pub enum MarkdownElement {
    Heading(u8, String),
    Paragraph(Vec<Inline>),
    CodeBlock(String, String),
    UnorderedList(Vec<String>),
    OrderedList(Vec<String>),
    Blockquote(String),
    HorizontalRule,
    LineBreak,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Inline {
    Text(String),
    Bold(String),
    Italic(String),
    Code(String),
    Link(String, String),
}

// Parse heading (# Header)
fn parse_heading(input: &str) -> IResult<&str, MarkdownElement> {
    let (input, hashes) = take_while1(|c| c == '#').parse(input)?;
    let level = hashes.len().min(6) as u8;
    let (input, _) = space0.parse(input)?;
    let (input, text) = not_line_ending.parse(input)?;
    let (input, _) = opt(line_ending).parse(input)?;

    Ok((
        input,
        MarkdownElement::Heading(level, text.trim().to_string()),
    ))
}

// Parse code block (```lang ... ```)
fn parse_code_block(input: &str) -> IResult<&str, MarkdownElement> {
    let (input, _) = tag("```").parse(input)?;
    let (input, lang) = not_line_ending.parse(input)?;
    let (input, _) = line_ending.parse(input)?;
    let (input, code) = take_until("```").parse(input)?;
    let (input, _) = tag("```").parse(input)?;
    let (input, _) = opt(line_ending).parse(input)?;

    Ok((
        input,
        MarkdownElement::CodeBlock(lang.trim().to_string(), code.to_string()),
    ))
}

// Parse unordered list item (- item or * item)
fn parse_unordered_item(input: &str) -> IResult<&str, String> {
    let (input, _) = alt((tag("- "), tag("* "))).parse(input)?;
    let (input, text) = not_line_ending.parse(input)?;
    let (input, _) = opt(line_ending).parse(input)?;

    Ok((input, text.to_string()))
}

// Parse unordered list
fn parse_unordered_list(input: &str) -> IResult<&str, MarkdownElement> {
    let (input, items) = many1(parse_unordered_item).parse(input)?;

    Ok((input, MarkdownElement::UnorderedList(items)))
}

// Parse ordered list item (1. item)
fn parse_ordered_item(input: &str) -> IResult<&str, String> {
    let (input, _) = digit1.parse(input)?;
    let (input, _) = tag(". ").parse(input)?;
    let (input, text) = not_line_ending.parse(input)?;
    let (input, _) = opt(line_ending).parse(input)?;

    Ok((input, text.to_string()))
}

// Parse ordered list
fn parse_ordered_list(input: &str) -> IResult<&str, MarkdownElement> {
    let (input, items) = many1(parse_ordered_item).parse(input)?;

    Ok((input, MarkdownElement::OrderedList(items)))
}

// Parse blockquote (> text)
fn parse_blockquote_line(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("> ").parse(input)?;
    let (input, text) = not_line_ending.parse(input)?;
    let (input, _) = opt(line_ending).parse(input)?;

    Ok((input, text.to_string()))
}

fn parse_blockquote(input: &str) -> IResult<&str, MarkdownElement> {
    let (input, lines) = many1(parse_blockquote_line).parse(input)?;

    Ok((input, MarkdownElement::Blockquote(lines.join("\n"))))
}

// Parse horizontal rule (---, ***, ___)
fn parse_horizontal_rule(input: &str) -> IResult<&str, MarkdownElement> {
    let (input, _) = alt((tag("---"), tag("***"), tag("___"))).parse(input)?;
    let (input, _) = not_line_ending.parse(input)?;
    let (input, _) = opt(line_ending).parse(input)?;

    Ok((input, MarkdownElement::HorizontalRule))
}

// Parse inline bold (**text**)
fn parse_bold(input: &str) -> IResult<&str, Inline> {
    let (input, text) = delimited(tag("**"), take_until("**"), tag("**")).parse(input)?;

    Ok((input, Inline::Bold(text.to_string())))
}

// Parse inline italic (*text*)
fn parse_italic(input: &str) -> IResult<&str, Inline> {
    let (input, text) = delimited(char('*'), take_while(|c| c != '*'), char('*')).parse(input)?;

    Ok((input, Inline::Italic(text.to_string())))
}

// Parse inline code (`code`)
fn parse_inline_code(input: &str) -> IResult<&str, Inline> {
    let (input, text) = delimited(char('`'), take_until("`"), char('`')).parse(input)?;

    Ok((input, Inline::Code(text.to_string())))
}

// Parse link ([text](url))
fn parse_link(input: &str) -> IResult<&str, Inline> {
    let (input, text) = delimited(char('['), take_until("]"), char(']')).parse(input)?;
    let (input, url) = delimited(char('('), take_until(")"), char(')')).parse(input)?;

    Ok((input, Inline::Link(text.to_string(), url.to_string())))
}

// Parse plain text
fn parse_text(input: &str) -> IResult<&str, Inline> {
    let (input, text) =
        take_while1(|c: char| c != '*' && c != '`' && c != '[' && c != '\n').parse(input)?;

    Ok((input, Inline::Text(text.to_string())))
}

// Parse inline elements
fn parse_inline(input: &str) -> IResult<&str, Inline> {
    alt((
        parse_bold,
        parse_inline_code,
        parse_link,
        parse_italic,
        parse_text,
    ))
    .parse(input)
}

// Parse paragraph
fn parse_paragraph(input: &str) -> IResult<&str, MarkdownElement> {
    let (input, line) = not_line_ending.parse(input)?;
    let (input, _) = opt(line_ending).parse(input)?;

    if line.is_empty() {
        return Ok((input, MarkdownElement::LineBreak));
    }

    // Try to parse inline elements, but if it fails, just treat as plain text
    let inlines = match many1(parse_inline).parse(line) {
        Ok((_, inlines)) => inlines,
        Err(_) => vec![Inline::Text(line.to_string())],
    };

    Ok((input, MarkdownElement::Paragraph(inlines)))
}

// Parse any markdown element
fn parse_element(input: &str) -> IResult<&str, MarkdownElement> {
    alt((
        parse_code_block,
        parse_heading,
        parse_horizontal_rule,
        parse_unordered_list,
        parse_ordered_list,
        parse_blockquote,
        parse_paragraph,
    ))
    .parse(input)
}

// Parse entire markdown document
pub fn parse_markdown(input: &str) -> IResult<&str, Vec<MarkdownElement>> {
    let mut elements = Vec::new();
    let mut remaining = input;

    loop {
        // Skip any leading whitespace
        let (input, _) = multispace0.parse(remaining)?;

        // If we've consumed all input, break
        if input.is_empty() {
            remaining = input;
            break;
        }

        // Try to parse an element
        match parse_element.parse(input) {
            Ok((rest, element)) => {
                elements.push(element);
                remaining = rest;
            }
            Err(_) => {
                // If we can't parse anything, break
                remaining = input;
                break;
            }
        }
    }

    Ok((remaining, elements))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown_heading() {
        let input = "# Hello World\n";
        let result = parse_heading.parse(input);
        assert_eq!(
            result,
            Ok(("", MarkdownElement::Heading(1, "Hello World".to_string())))
        );
    }

    #[test]
    fn test_parse_markdown_multiple_heading_levels() {
        assert_eq!(
            parse_heading.parse("## Level 2\n"),
            Ok(("", MarkdownElement::Heading(2, "Level 2".to_string())))
        );
        assert_eq!(
            parse_heading.parse("### Level 3\n"),
            Ok(("", MarkdownElement::Heading(3, "Level 3".to_string())))
        );
    }

    #[test]
    fn test_parse_markdown_bold() {
        let input = "**bold text**";
        let result = parse_bold.parse(input);
        assert_eq!(result, Ok(("", Inline::Bold("bold text".to_string()))));
    }

    #[test]
    fn test_parse_markdown_italic() {
        let input = "*italic text*";
        let result = parse_italic.parse(input);
        assert_eq!(result, Ok(("", Inline::Italic("italic text".to_string()))));
    }

    #[test]
    fn test_parse_markdown_inline_code() {
        let input = "`code`";
        let result = parse_inline_code.parse(input);
        assert_eq!(result, Ok(("", Inline::Code("code".to_string()))));
    }

    #[test]
    fn test_parse_markdown_link() {
        let input = "[Rust](https://rust-lang.org)";
        let result = parse_link.parse(input);
        assert_eq!(
            result,
            Ok((
                "",
                Inline::Link("Rust".to_string(), "https://rust-lang.org".to_string())
            ))
        );
    }

    #[test]
    fn test_parse_markdown_unordered_list() {
        let input = "- Item 1\n- Item 2\n- Item 3\n";
        let result = parse_unordered_list.parse(input);
        assert_eq!(
            result,
            Ok((
                "",
                MarkdownElement::UnorderedList(vec![
                    "Item 1".to_string(),
                    "Item 2".to_string(),
                    "Item 3".to_string()
                ])
            ))
        );
    }

    #[test]
    fn test_parse_markdown_ordered_list() {
        let input = "1. First\n2. Second\n3. Third\n";
        let result = parse_ordered_list.parse(input);
        assert_eq!(
            result,
            Ok((
                "",
                MarkdownElement::OrderedList(vec![
                    "First".to_string(),
                    "Second".to_string(),
                    "Third".to_string()
                ])
            ))
        );
    }

    #[test]
    fn test_parse_markdown_blockquote() {
        let input = "> Quote line 1\n> Quote line 2\n";
        let result = parse_blockquote.parse(input);
        assert_eq!(
            result,
            Ok((
                "",
                MarkdownElement::Blockquote("Quote line 1\nQuote line 2".to_string())
            ))
        );
    }

    #[test]
    fn test_parse_markdown_code_block() {
        let input = "```rust\nfn main() {}\n```\n";
        let result = parse_code_block.parse(input);
        assert_eq!(
            result,
            Ok((
                "",
                MarkdownElement::CodeBlock("rust".to_string(), "fn main() {}\n".to_string())
            ))
        );
    }

    #[test]
    fn test_parse_markdown_horizontal_rule() {
        assert_eq!(
            parse_horizontal_rule.parse("---\n"),
            Ok(("", MarkdownElement::HorizontalRule))
        );
        assert_eq!(
            parse_horizontal_rule.parse("***\n"),
            Ok(("", MarkdownElement::HorizontalRule))
        );
        assert_eq!(
            parse_horizontal_rule.parse("___\n"),
            Ok(("", MarkdownElement::HorizontalRule))
        );
    }

    #[test]
    fn test_parse_markdown_paragraph_with_inline_elements() {
        let input = "This is **bold** and *italic*\n";
        let result = parse_paragraph.parse(input);
        assert!(matches!(result, Ok((_, MarkdownElement::Paragraph(_)))));

        if let Ok((_, MarkdownElement::Paragraph(inlines))) = result {
            assert_eq!(inlines.len(), 4);
            assert_eq!(inlines[0], Inline::Text("This is ".to_string()));
            assert_eq!(inlines[1], Inline::Bold("bold".to_string()));
            assert_eq!(inlines[2], Inline::Text(" and ".to_string()));
            assert_eq!(inlines[3], Inline::Italic("italic".to_string()));
        }
    }

    #[test]
    fn test_parse_markdown_full_document() {
        let markdown = r#"# Title

This is a **paragraph** with `code`.

- List item 1
- List item 2

```rust
fn main() {}
```
"#;

        let result = parse_markdown.parse(markdown);
        println!("{:?}", result);
        assert!(result.is_ok(), "Parse failed: {:?}", result);

        if let Ok((_, elements)) = result {
            // Filter out LineBreak elements for easier testing
            let non_breaks: Vec<_> = elements
                .iter()
                .filter(|e| !matches!(e, MarkdownElement::LineBreak))
                .collect();

            assert!(
                non_breaks.len() >= 4,
                "Expected at least 4 non-linebreak elements, got {}",
                non_breaks.len()
            );
            assert!(matches!(non_breaks[0], MarkdownElement::Heading(1, _)));
            assert!(matches!(non_breaks[1], MarkdownElement::Paragraph(_)));
            assert!(matches!(non_breaks[2], MarkdownElement::UnorderedList(_)));
            assert!(matches!(non_breaks[3], MarkdownElement::CodeBlock(_, _)));
        }
    }
}
