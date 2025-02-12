// Syntax highlighting and tree sitter helpers
//
// Uses https://github.com/tree-sitter-grammars/tree-sitter-markdown/ which
// splits the markdown parser into a main document and then another for
// inline languages for code blocks.
//
// This will require usage of the Highlighter trait.
use std::error::Error;
use tree_sitter::Language;

#[derive(Debug)]
enum MDNode {
    Document(Vec<MDNode>),
    Heading {
        level: u8,
        content: Vec<MDNode>,
    },
    Paragraph(Vec<MDNode>),
    CodeBlock {
        language: Option<String>,
        content: String,
    },
    InlineCode(String),
    Bold(Vec<MDNode>),
    Em(Vec<MDNode>),
    Link {
        text: Vec<MDNode>,
        url: String,
    },
    Text(String),
    List {
        ordered: bool,
        items: Vec<Vec<MDNode>>,
    },
    BlockQuote(Vec<MDNode>),
}

pub struct MarkdownParser {
    inner: tree_sitter::Parser,
    lang: Language,
}

impl MarkdownParser {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut inner = tree_sitter::Parser::new();
        extern "C" {
            fn tree_sitter_markdown() -> Language;
        }
        let lang = unsafe { tree_sitter_markdown() };

        inner.set_language(&lang)?;

        Ok(Self { inner, lang })
    }

    pub fn parse(&mut self, content: &str) -> Result<MDNode, Box<dyn Error>> {
        let tree = self
            .inner
            .parse(content, None)
            .ok_or("failed to build tree from contents")?;

        Ok(self.parse_blocks(&tree.root_node(), content))
    }

    fn parse_blocks(&mut self, node: &tree_sitter::Node, content: &str) -> MDNode {
        match node.kind() {
            "document" => {
                let children = node
                    .named_children(&mut node.walk())
                    .map(|child| self.parse_blocks(&child, content))
                    .collect();
                MDNode::Document(children)
            }
            "atx_heading" | "setext_heading" => {
                let level = if node.kind() == "atx_heading" {
                    node.child(0)
                        .map(|n| n.start_byte() - n.start_position().column)
                        .unwrap_or(1) as u8
                } else {
                    1
                };

                let heading_content = node
                    .named_children(&mut node.walk())
                    .filter(|n| n.kind() == "inline")
                    .flat_map(|n| self.parse_inline(&n, content))
                    .collect();

                MDNode::Heading {
                    level,
                    content: heading_content,
                }
            }
            "paragraph" => {
                let inline_content = node
                    .named_children(&mut node.walk())
                    .filter(|n| n.kind() == "inline")
                    .flat_map(|n| self.parse_inline(&n, content))
                    .collect();
                MDNode::Paragraph(inline_content)
            }
            "fenced_code_block" => {
                let info_string = node
                    .child_by_field_name("info")
                    .map(|n| content[n.byte_range()].trim().to_string());

                let code_content = node
                    .child_by_field_name("content")
                    .map(|n| content[n.byte_range()].to_string())
                    .unwrap_or_default();

                MDNode::CodeBlock {
                    language: info_string,
                    content: code_content,
                }
            }
            "list" => {
                let ordered = node
                    .child(0)
                    .map(|n| n.kind() == "ordered_list")
                    .unwrap_or(false);

                let items: Vec<Vec<MDNode>> = node
                    .named_children(&mut node.walk())
                    .filter(|n| n.kind() == "list_item")
                    .map(|item| {
                        item.named_children(&mut item.walk())
                            .map(|child| self.parse_blocks(&child, content))
                            .collect()
                    })
                    .collect();

                MDNode::List { ordered, items }
            }
            "block_quote" => {
                let quote_content = node
                    .named_children(&mut node.walk())
                    .map(|child| self.parse_blocks(&child, content))
                    .collect();
                MDNode::BlockQuote(quote_content)
            }
            _ => MDNode::Text(content[node.byte_range()].to_string()),
        }
    }

    fn parse_inline(&mut self, node: &tree_sitter::Node, content: &str) -> Vec<MDNode> {
        let _ = self
            .inner
            .parse(content, None)
            .expect("Failed to parse inline content");

        let mut nodes = Vec::new();
        for child in node.named_children(&mut node.walk()) {
            match child.kind() {
                "emphasis" => {
                    let emph_content = child
                        .named_children(&mut child.walk())
                        .flat_map(|n| self.parse_inline(&n, content))
                        .collect();
                    nodes.push(MDNode::Em(emph_content));
                }
                "strong_emphasis" => {
                    let strong_content = child
                        .named_children(&mut child.walk())
                        .flat_map(|n| self.parse_inline(&n, content))
                        .collect();
                    nodes.push(MDNode::Bold(strong_content));
                }
                "code_span" => {
                    let code =
                        content[child.byte_range()].trim_matches('`').to_string();
                    nodes.push(MDNode::InlineCode(code));
                }
                "link" => {
                    let text = child
                        .child_by_field_name("text")
                        .map(|n| self.parse_inline(&n, content))
                        .unwrap_or_default();

                    let url = child
                        .child_by_field_name("url")
                        .map(|n| content[n.byte_range()].to_string())
                        .unwrap_or_default();

                    nodes.push(MDNode::Link { text, url });
                }
                _ => {
                    if !child.is_extra() {
                        nodes.push(MDNode::Text(
                            content[child.byte_range()].to_string(),
                        ));
                    }
                }
            }
        }

        nodes
    }
}
