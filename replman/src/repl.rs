use rustyline::Editor;

use crate::parsing::split_string_unescape;
use crate::Replman;

pub struct Repl {
    editor: Editor<()>,
}

impl Repl {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            editor: Editor::new(),
        }
    }

    pub fn read_command<R>(&mut self) -> anyhow::Result<R>
    where
        R: Replman,
    {
        loop {
            let line = self.editor.readline("> ")?;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            match R::parse(split_string_unescape(trimmed)) {
                Ok(cmd) => {
                    self.editor.add_history_entry(trimmed);
                    return Ok(cmd);
                }
                Err(err) => eprintln!("Failed to parse command: {}", err),
            }
        }
    }
}
