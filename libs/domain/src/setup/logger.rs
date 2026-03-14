use crate::setup::logger::tracing_visitor::DetailedMessageVisitor;
use chrono::Utc;
use foxtive::FOXTIVE;
use foxtive::prelude::AppStateExt;
use tracing::Level;

pub const BLACKLISTED_MODULES: [&str; 4] = [
    "foxtive::enums::app_message",
    "foxtive_axum::http::extractors::json_body",
    "foxtive_axum::http::extractors::byte_body",
    "foxtive_axum::http::extractors::string_body",
];

pub fn on_event(event: &tracing::Event<'_>) {
    if FOXTIVE.is_initialized() {
        let mut visitor = DetailedMessageVisitor::default();
        event.record(&mut visitor);

        if matches!(event.metadata().level(), &Level::ERROR) {
            if !can_process_log(event, &visitor) {
                return;
            }

            let _detailed_message = format_error_message(event, &visitor);
        }
    }
}

fn can_process_log(event: &tracing::Event<'_>, visitor: &DetailedMessageVisitor) -> bool {
    let blacklisted = BLACKLISTED_MODULES;

    let target = event.metadata().target().to_string();
    if blacklisted.contains(&target.as_str()) {
        return false;
    }

    if let Some(target) = visitor.fields.get("log.target")
        && blacklisted.contains(&target.as_str())
    {
        return false;
    }

    true
}

fn format_error_message(event: &tracing::Event<'_>, visitor: &DetailedMessageVisitor) -> String {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    let metadata = event.metadata();

    let mut message = String::new();
    message.push_str("🚨 <b>ERROR ALERT</b>\n\n");

    // Timestamp
    message.push_str(&format!(
        "<b>Time:</b> {}\n",
        html_escape(&timestamp.to_string())
    ));

    message.push_str(&format!(
        "<b>Level:</b> {}\n",
        html_escape(&metadata.level().to_string())
    ));

    // Target/Module
    message.push_str(&format!(
        "<b>Module:</b> <code>{}</code>\n",
        html_escape(metadata.target())
    ));

    // File and line if available
    if let Some(file) = metadata.file() {
        let line = metadata.line().map(|l| l.to_string()).unwrap_or_default();
        message.push_str(&format!(
            "<b>Location:</b> <code>{}:{}</code>\n",
            html_escape(file),
            html_escape(&line)
        ));
    }

    message.push('\n');

    // Main error message
    if let Some(error_msg) = &visitor.message {
        message.push_str(&format!(
            "<b>Error Message:</b>\n<pre>{}</pre>\n\n",
            html_escape(error_msg)
        ));
    }

    // Additional fields
    if !visitor.fields.is_empty() {
        message.push_str("<b>Additional Context:</b>\n");
        for (key, value) in &visitor.fields {
            if key != "message" {
                message.push_str(&format!(
                    "• <b>{}:</b> <code>{}</code>\n",
                    html_escape(key),
                    html_escape(value)
                ));
            }
        }
        message.push('\n');
    }

    // Error chain if available
    if let Some(error_chain) = &visitor.error_chain {
        message.push_str(&format!(
            "<b>Error Chain:</b>\n<pre>{}</pre>\n\n",
            html_escape(error_chain)
        ));
    }

    // Backtrace if available
    if let Some(backtrace) = &visitor.backtrace {
        message.push_str(&format!(
            "<b>Backtrace:</b>\n<pre>{}</pre>\n\n",
            html_escape(&truncate_backtrace(backtrace, 1000))
        )); // Limit backtrace length
    }

    // Process info
    message.push_str(&format!("<b>PID:</b> {}\n", std::process::id()));

    // System info
    message.push_str(&format!(
        "<b>System:</b> {}\n",
        html_escape(&std::env::var("HOSTNAME").unwrap_or_else(|_| "Unknown".to_string()))
    ));

    // Environment
    message.push_str(&format!("<b>Environment:</b> {}\n", FOXTIVE.env()));

    message.push_str("\n<i>───────────────────</i>\n");
    message.push_str(&format!(
        "<i>This is an automated error report from {} service.</i>",
        FOXTIVE.app_code()
    ));

    message
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn truncate_backtrace(backtrace: &str, max_length: usize) -> String {
    if backtrace.len() <= max_length {
        backtrace.to_string()
    } else {
        let truncated = &backtrace[..max_length];
        format!(
            "{}...\n\n[Backtrace truncated - original length: {} characters]",
            truncated,
            backtrace.len()
        )
    }
}

mod tracing_visitor {
    use std::collections::HashMap;
    use std::fmt;
    use tracing::field::{Field, Visit};

    #[derive(Default, Debug)]
    pub struct DetailedMessageVisitor {
        pub message: Option<String>,
        pub fields: HashMap<String, String>,
        pub error_chain: Option<String>,
        pub backtrace: Option<String>,
    }

    impl Visit for DetailedMessageVisitor {
        fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
            let field_name = field.name();
            let field_value = format!("{value:?}");

            match field_name {
                "message" => {
                    self.message = Some(field_value);
                }
                "error" => {
                    // Try to extract error chain
                    self.error_chain = Some(field_value.clone());
                    self.fields.insert(field_name.to_string(), field_value);
                }
                "backtrace" => {
                    self.backtrace = Some(field_value.clone());
                    self.fields.insert(field_name.to_string(), field_value);
                }
                _ => {
                    self.fields.insert(field_name.to_string(), field_value);
                }
            }
        }

        fn record_str(&mut self, field: &Field, value: &str) {
            let field_name = field.name();

            match field_name {
                "message" => {
                    self.message = Some(value.to_string());
                }
                "error" => {
                    self.error_chain = Some(value.to_string());
                    self.fields
                        .insert(field_name.to_string(), value.to_string());
                }
                "backtrace" => {
                    self.backtrace = Some(value.to_string());
                    self.fields
                        .insert(field_name.to_string(), value.to_string());
                }
                _ => {
                    self.fields
                        .insert(field_name.to_string(), value.to_string());
                }
            }
        }

        fn record_i64(&mut self, field: &Field, value: i64) {
            self.fields
                .insert(field.name().to_string(), value.to_string());
        }

        fn record_u64(&mut self, field: &Field, value: u64) {
            self.fields
                .insert(field.name().to_string(), value.to_string());
        }

        fn record_bool(&mut self, field: &Field, value: bool) {
            self.fields
                .insert(field.name().to_string(), value.to_string());
        }
    }
}
