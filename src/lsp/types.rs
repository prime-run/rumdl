//! LSP type definitions and utilities for rumdl
//!
//! This module contains LSP-specific types and utilities for rumdl,
//! following the Language Server Protocol specification.

use serde::{Deserialize, Serialize};
use tower_lsp::lsp_types::*;

/// Configuration for the rumdl LSP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RumdlLspConfig {
    /// Path to rumdl configuration file
    pub config_path: Option<String>,
    /// Enable/disable real-time linting
    pub enable_linting: bool,
    /// Enable/disable auto-fixing on save
    pub enable_auto_fix: bool,
    /// Rules to disable in the LSP server
    pub disable_rules: Vec<String>,
}

impl Default for RumdlLspConfig {
    fn default() -> Self {
        Self {
            config_path: None,
            enable_linting: true,
            enable_auto_fix: false,
            disable_rules: Vec::new(),
        }
    }
}

/// Convert rumdl warnings to LSP diagnostics
pub fn warning_to_diagnostic(warning: &crate::rule::LintWarning) -> Diagnostic {
    let start_position = Position {
        line: (warning.line.saturating_sub(1)) as u32,
        character: (warning.column.saturating_sub(1)) as u32,
    };

    // Use proper range from warning
    let end_position = Position {
        line: (warning.end_line.saturating_sub(1)) as u32,
        character: (warning.end_column.saturating_sub(1)) as u32,
    };

    let severity = match warning.severity {
        crate::rule::Severity::Error => DiagnosticSeverity::ERROR,
        crate::rule::Severity::Warning => DiagnosticSeverity::WARNING,
    };

    // Create clickable link to rule documentation
    let code_description = warning
        .rule_name
        .as_ref()
        .and_then(|rule_name| {
            // Create a link to the rule documentation
            Url::parse(&format!(
                "https://github.com/rvben/rumdl/blob/main/docs/{}.md",
                rule_name.to_lowercase()
            ))
            .ok()
            .map(|href| CodeDescription { href })
        });

    Diagnostic {
        range: Range {
            start: start_position,
            end: end_position,
        },
        severity: Some(severity),
        code: warning
            .rule_name
            .map(|s| NumberOrString::String(s.to_string())),
        source: Some("rumdl".to_string()),
        message: warning.message.clone(),
        related_information: None,
        tags: None,
        code_description,
        data: None,
    }
}

/// Convert byte range to LSP range
fn byte_range_to_lsp_range(text: &str, byte_range: std::ops::Range<usize>) -> Option<Range> {
    let mut line = 0u32;
    let mut character = 0u32;
    let mut byte_pos = 0;

    let mut start_pos = None;
    let mut end_pos = None;

    for ch in text.chars() {
        if byte_pos == byte_range.start {
            start_pos = Some(Position { line, character });
        }
        if byte_pos == byte_range.end {
            end_pos = Some(Position { line, character });
            break;
        }

        if ch == '\n' {
            line += 1;
            character = 0;
        } else {
            character += 1;
        }

        byte_pos += ch.len_utf8();
    }

    // Handle end position at EOF
    if byte_pos == byte_range.end && end_pos.is_none() {
        end_pos = Some(Position { line, character });
    }

    match (start_pos, end_pos) {
        (Some(start), Some(end)) => Some(Range { start, end }),
        _ => None,
    }
}

/// Create a code action from a rumdl warning with fix
pub fn warning_to_code_action(
    warning: &crate::rule::LintWarning,
    uri: &Url,
    document_text: &str,
) -> Option<CodeAction> {
    if let Some(fix) = &warning.fix {
        // Convert fix range (byte offsets) to LSP positions
        let range = byte_range_to_lsp_range(document_text, fix.range.clone())?;

        let edit = TextEdit {
            range,
            new_text: fix.replacement.clone(),
        };

        let mut changes = std::collections::HashMap::new();
        changes.insert(uri.clone(), vec![edit]);

        let workspace_edit = WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        };

        Some(CodeAction {
            title: format!("Fix: {}", warning.message),
            kind: Some(CodeActionKind::QUICKFIX),
            diagnostics: Some(vec![warning_to_diagnostic(warning)]),
            edit: Some(workspace_edit),
            command: None,
            is_preferred: Some(true),
            disabled: None,
            data: None,
        })
    } else {
        None
    }
}
