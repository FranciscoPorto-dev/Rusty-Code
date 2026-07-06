use crate::model::App;

impl App {
    fn clamp_cursor(&mut self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }
    
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left= self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }
    
    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn move_cursor_to_start(&mut self) {
        self.character_index = 0;
    }
}