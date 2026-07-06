use crate::model::App;

impl App {
    pub fn enter_char(&mut self, new_char: char) {
        self.save_edit_state();
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            self.save_edit_state();

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn delete_to_line_start(&mut self) {
        if self.character_index == 0 {
            return;
        }

        self.save_edit_state();
        let byte_index = self.byte_index();
        self.input.drain(..byte_index);
        self.character_index = 0;
    }

    pub fn undo(&mut self) {
        if let Some((input, character_index)) = self.edit_history.pop() {
            self.input = input;
            self.character_index = character_index;
        }
    }

    pub fn clear_input(&mut self) {
        if self.input.is_empty() {
            return;
        }

        self.save_edit_state();
        self.input.clear();
        self.reset_cursor();
    }

    pub fn paste(&mut self) {
        let Ok(mut clipboard) = arboard::Clipboard::new() else {
            return;
        };

        let Ok(text) = clipboard.get_text() else {
            return;
        };

        self.paste_text(&text);
    }

    fn paste_text(&mut self, text: &str) {
        let text: String = text.chars().filter(|c| *c != '\n' && *c != '\r').collect();
        if text.is_empty() {
            return;
        }

        self.save_edit_state();
        let index = self.byte_index();
        self.input.insert_str(index, &text);
        self.character_index += text.chars().count();
    }

    pub fn submit_message(&mut self) {
        self.input.clear();
        self.reset_cursor();
        self.edit_history.clear();
    }

    pub fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn save_edit_state(&mut self) {
        self.edit_history
            .push((self.input.clone(), self.character_index));
    }
}
