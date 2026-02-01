use super::frame::SSEFrame;
use crate::consts::client::malexp::events;

pub(crate) struct SSEParser {
    buf: String,
    curr: SSEFrame,
}

impl SSEParser {
    pub(crate) fn new() -> Self {
        Self {
            buf: String::new(),
            curr: SSEFrame::default(),
        }
    }
    pub(crate) fn push(&mut self, text: &str) -> Vec<SSEFrame> {
        self.buf.push_str(text);
        let mut out = Vec::new();

        while let Some(pos) = self.buf.find('\n') {
            let mut line = self.buf[..pos].to_string();
            self.buf.drain(..=pos);

            if line.ends_with('\r') {
                line.pop();
            }

            if line.is_empty() {
                if self.curr.id.is_some()
                    || self.curr.event.is_some()
                    || !self.curr.data.is_empty()
                {
                    out.push(std::mem::take(&mut self.curr));
                }
                continue;
            }

            if let Some(v) = line.strip_prefix(events::ID_PREFIX) {
                self.curr.id = Some(v.trim().to_string());
            } else if let Some(v) = line.strip_prefix(events::EVENT_PREFIX) {
                self.curr.event = Some(v.trim().to_string());
            } else if let Some(v) = line.strip_prefix(events::DATA_PREFIX) {
                if !self.curr.data.is_empty() {
                    self.curr.data.push('\n');
                }
                self.curr.data.push_str(v.trim());
            }
        }

        out
    }
}
