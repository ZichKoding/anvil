use std::path::Path;
use tree_sitter::{Node, Parser, Tree};

use super::languages::LangId;

#[derive(Debug, Clone, Copy)]
pub enum HighlightGroup {
    Keyword,
    String,
    Comment,
    Function,
    Type,
    Number,
    Operator,
    Punctuation,
    Variable,
    Constant,
    Property,
    Normal,
}

pub struct SyntaxHighlighter {
    parser: Parser,
    tree: Option<Tree>,
    lang_id: Option<LangId>,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            tree: None,
            lang_id: None,
        }
    }

    pub fn set_language_from_path(&mut self, path: &Path) {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        self.lang_id = LangId::from_extension(ext);
        if let Some(lang_id) = &self.lang_id {
            let _ = self.parser.set_language(&lang_id.language());
        }
    }

    pub fn parse(&mut self, source: &str) {
        if self.lang_id.is_some() {
            self.tree = self.parser.parse(source, self.tree.as_ref());
        }
    }

    pub fn lang_name(&self) -> &str {
        self.lang_id.map(|l| l.name()).unwrap_or("Plain Text")
    }

    pub fn highlight_line(
        &self,
        line_byte_start: usize,
        line_byte_end: usize,
    ) -> Vec<(usize, usize, HighlightGroup)> {
        let mut spans = Vec::new();

        let Some(tree) = &self.tree else {
            return spans;
        };

        let root = tree.root_node();
        collect_leaf_spans(&root, line_byte_start, line_byte_end, &mut spans);

        // Sort by start position
        spans.sort_by_key(|(start, _, _)| *start);
        spans
    }
}

fn collect_leaf_spans(
    node: &Node,
    line_start: usize,
    line_end: usize,
    spans: &mut Vec<(usize, usize, HighlightGroup)>,
) {
    let node_start = node.start_byte();
    let node_end = node.end_byte();

    // Skip nodes entirely outside our line range
    if node_end <= line_start || node_start >= line_end {
        return;
    }

    if node.child_count() == 0 {
        // Leaf node - map to highlight group, passing parent kind for context
        let parent_kind = node.parent().map(|p| p.kind());
        let group = node_kind_to_group(node.kind(), parent_kind);
        let span_start = node_start.max(line_start) - line_start;
        let span_end = node_end.min(line_end) - line_start;
        if span_start < span_end {
            spans.push((span_start, span_end, group));
        }
    } else {
        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            collect_leaf_spans(&child, line_start, line_end, spans);
        }
    }
}

fn node_kind_to_group(kind: &str, parent_kind: Option<&str>) -> HighlightGroup {
    match kind {
        // Keywords
        "fn" | "let" | "mut" | "pub" | "use" | "mod" | "struct" | "enum" | "impl" |
        "trait" | "for" | "while" | "loop" | "if" | "else" | "match" | "return" |
        "break" | "continue" | "as" | "in" | "where" | "type" | "const" | "static" |
        "ref" | "self" | "super" | "crate" | "async" | "await" | "move" | "unsafe" |
        "extern" | "dyn" |
        // Python keywords
        "def" | "class" | "import" | "from" | "try" | "except" | "finally" |
        "with" | "yield" | "lambda" | "pass" | "raise" | "global" | "nonlocal" |
        "assert" | "del" | "is" | "not" | "and" | "or" | "elif" |
        // JS keywords
        "function" | "var" | "new" | "this" | "typeof" | "instanceof" |
        "switch" | "case" | "default" | "throw" | "catch" | "export" |
        "extends" | "of" | "void" | "delete" | "debugger" |
        // Common
        "true" | "false" | "None" | "null" | "undefined" => HighlightGroup::Keyword,

        // Strings
        "string_literal" | "string_content" | "raw_string_literal" |
        "char_literal" | "string" | "template_string" | "string_fragment" |
        "interpreted_string_literal" | "escape_sequence" => HighlightGroup::String,

        // Comments
        "line_comment" | "block_comment" | "comment" => HighlightGroup::Comment,

        // Numbers
        "integer_literal" | "float_literal" | "number" | "integer" | "float" => HighlightGroup::Number,

        // Types
        "type_identifier" | "primitive_type" | "generic_type" |
        "scoped_type_identifier" => HighlightGroup::Type,

        // Identifiers — context-aware classification
        "identifier" => match parent_kind {
            Some("call_expression" | "function_item" | "function_definition" | "function_declaration") => {
                HighlightGroup::Function
            }
            Some("const_item") => HighlightGroup::Constant,
            _ => HighlightGroup::Variable,
        },
        "field_identifier" | "property_identifier" => HighlightGroup::Property,

        // Operators
        "+" | "-" | "*" | "/" | "%" | "=" | "==" | "!=" | "<" | ">" |
        "<=" | ">=" | "&&" | "||" | "!" | "&" | "|" | "^" | "~" |
        "<<" | ">>" | "+=" | "-=" | "*=" | "/=" | "=>" | "->" | "::" => HighlightGroup::Operator,

        // Punctuation
        "(" | ")" | "[" | "]" | "{" | "}" | "," | ";" | ":" | "." => HighlightGroup::Punctuation,

        _ => HighlightGroup::Normal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- node_kind_to_group: identifier context-aware classification ---

    #[test]
    fn test_identifier_in_call_expression_is_function() {
        assert!(matches!(
            node_kind_to_group("identifier", Some("call_expression")),
            HighlightGroup::Function
        ));
    }

    #[test]
    fn test_identifier_in_function_item_is_function() {
        assert!(matches!(
            node_kind_to_group("identifier", Some("function_item")),
            HighlightGroup::Function
        ));
    }

    #[test]
    fn test_identifier_in_function_definition_is_function() {
        assert!(matches!(
            node_kind_to_group("identifier", Some("function_definition")),
            HighlightGroup::Function
        ));
    }

    #[test]
    fn test_identifier_in_function_declaration_is_function() {
        assert!(matches!(
            node_kind_to_group("identifier", Some("function_declaration")),
            HighlightGroup::Function
        ));
    }

    #[test]
    fn test_identifier_in_const_item_is_constant() {
        assert!(matches!(
            node_kind_to_group("identifier", Some("const_item")),
            HighlightGroup::Constant
        ));
    }

    #[test]
    fn test_identifier_with_no_parent_is_variable() {
        assert!(matches!(
            node_kind_to_group("identifier", None),
            HighlightGroup::Variable
        ));
    }

    #[test]
    fn test_identifier_in_let_declaration_is_variable() {
        assert!(matches!(
            node_kind_to_group("identifier", Some("let_declaration")),
            HighlightGroup::Variable
        ));
    }

    // --- node_kind_to_group: existing mappings unchanged ---

    #[test]
    fn test_keyword_fn() {
        assert!(matches!(
            node_kind_to_group("fn", None),
            HighlightGroup::Keyword
        ));
    }

    #[test]
    fn test_keyword_let() {
        assert!(matches!(
            node_kind_to_group("let", None),
            HighlightGroup::Keyword
        ));
    }

    #[test]
    fn test_string_literal() {
        assert!(matches!(
            node_kind_to_group("string_literal", None),
            HighlightGroup::String
        ));
    }

    #[test]
    fn test_line_comment() {
        assert!(matches!(
            node_kind_to_group("line_comment", None),
            HighlightGroup::Comment
        ));
    }

    #[test]
    fn test_integer_literal() {
        assert!(matches!(
            node_kind_to_group("integer_literal", None),
            HighlightGroup::Number
        ));
    }

    #[test]
    fn test_type_identifier() {
        assert!(matches!(
            node_kind_to_group("type_identifier", None),
            HighlightGroup::Type
        ));
    }

    #[test]
    fn test_field_identifier_is_property() {
        assert!(matches!(
            node_kind_to_group("field_identifier", None),
            HighlightGroup::Property
        ));
    }

    #[test]
    fn test_plus_operator() {
        assert!(matches!(
            node_kind_to_group("+", None),
            HighlightGroup::Operator
        ));
    }

    #[test]
    fn test_paren_punctuation() {
        assert!(matches!(
            node_kind_to_group("(", None),
            HighlightGroup::Punctuation
        ));
    }

    #[test]
    fn test_unknown_kind_is_normal() {
        assert!(matches!(
            node_kind_to_group("some_unknown_kind", None),
            HighlightGroup::Normal
        ));
    }
}
