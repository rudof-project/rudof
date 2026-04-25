use crate::display::Color;
use colored::{ColoredString, Colorize};
use prefixmap::PrefixMap;
use shacl::validator::report::ValidationReport;
use std::io::{Error, Write};
use tabled::builder::Builder;
use tabled::settings::object::Segment;
use tabled::settings::{Modify, Style, Width};

// Maybe this could be implemented as a builder?
pub trait Table {
    // Maybe add sort mode
    fn table<W: Write>(
        &self,
        writer: W,
        detailed: Option<bool>,
        colored: Option<bool>,
        termial_width: Option<usize>,
    ) -> Result<(), std::io::Error>;
}

impl Table for ValidationReport {
    fn table<W: Write>(
        &self,
        mut writer: W,
        detailed: Option<bool>,
        colored: Option<bool>,
        terminal_width: Option<usize>,
    ) -> Result<(), Error> {
        if self.results().is_empty() {
            return write!(writer, "No Errors found");
        }

        let detailed = detailed.unwrap_or(false);
        let terminal_width = terminal_width.unwrap_or(80);
        let colored = colored.unwrap_or(false);

        let mut builder = Builder::default();
        let mut header = vec![
            "Severity",
            "Node",
            "Component",
            "Path",
            "Value",
            "Source shape",
            "Details",
        ];
        if detailed {
            header.push("Details");
        }
        builder.push_record(header);

        let pm = if colored {
            PrefixMap::basic()
        } else {
            PrefixMap::basic().with_hyperlink(true).without_default_colors()
        };

        for result in self.results() {
            let severity_str = pm.qualify(&result.severity().into());
            let severity = match colored {
                true => severity_str.color(result.severity().color()),
                false => ColoredString::from(severity_str),
            }
            .to_string();
            let node = self.nodes_prefixmap().show(result.focus_node());
            let component = self.nodes_prefixmap().show(result.constraint_component());
            let path = self.nodes_prefixmap().show(&result.path());
            let source = self.nodes_prefixmap().show(&result.source());
            let value = self.nodes_prefixmap().show(&result.value());
            let details: String;

            let mut record = vec![&severity, &node, &component, &path, &value, &source];

            if detailed {
                details = result
                    .message()
                    .iter()
                    .fold(String::new(), |acc, (lang, msg)| match lang {
                        None => format!("{}- {}\n", acc, msg),
                        Some(lang) => format!("{}- {}: {}\n", acc, lang, msg),
                    });
                record.push(&details);
            }
            builder.push_record(record);
        }
        let table = builder
            .build()
            .with(Style::modern_rounded())
            .with(Modify::new(Segment::all()).with(Width::wrap(terminal_width)))
            .to_string();
        write!(writer, "{table}")
    }
}
