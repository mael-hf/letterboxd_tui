use crate::models::{Film, Filter};

pub enum NamingMode {
    Creating,
    Modifying,
}

pub enum InputMode {
    Normal,
    Rating,
    Director,
    Naming(NamingMode),
    Breakdown,
}

pub struct App {
    pub films: Vec<Film>,
    pub selected_index: usize,
    pub filter: Filter,
    pub input_mode: InputMode,
    pub input_value: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            films: Vec::new(),
            selected_index: 0,
            filter: Filter::default(),
            input_mode: InputMode::Normal,
            input_value: String::from(""),
        }
    }

    pub fn filtered_films(&self) -> Vec<&Film> {
        match self.filter {
            Filter::All => self.films.iter().collect(),
            Filter::Unwatched => self.films.iter().filter(|f| !f.watched).collect(),
            Filter::Watched => self.films.iter().filter(|f| f.watched).collect(),
        }
    }

    pub fn move_selection_up(&mut self) {
        if !self.filtered_films().is_empty() {
            self.selected_index = self.selected_index.saturating_sub(1);
        }
    }

    pub fn move_selection_down(&mut self) {
        let len = self.filtered_films().len();
        if len > 0 && self.selected_index < len - 1 {
            self.selected_index += 1;
        }
    }

    pub fn add_film(&mut self, name: String) {
        let film = Film::new(name);
        self.films.push(film);
    }

    pub fn toggle_watched(&mut self) {
        if let Some(film) = self.filtered_films().get(self.selected_index) {
            if let Some(idx) = self.films.iter().position(|f| f.name == film.name) {
                self.films[idx].watched = !self.films[idx].watched;
            }
        }
    }

    pub fn rate(&mut self, rating: u8) {
        if let Some(film) = self.filtered_films().get(self.selected_index) {
            if let Some(idx) = self.films.iter().position(|f| f.name == film.name) {
                self.films[idx].rating = Some(rating);
            }
        }
    }

    pub fn director(&mut self, director: String) {
        if let Some(film) = self.filtered_films().get(self.selected_index) {
            if let Some(idx) = self.films.iter().position(|f| f.name == film.name) {
                self.films[idx].director = Some(director);
            }
        }
    }

    pub fn rename(&mut self, new_name: String) {
        if let Some(film) = self.filtered_films().get(self.selected_index) {
            if let Some(idx) = self.films.iter().position(|f| f.name == film.name) {
                self.films[idx].name = new_name;
            }
        }
    }

    pub fn delete_film(&mut self) {
        if let Some(film) = self.filtered_films().get(self.selected_index) {
            if let Some(idx) = self.films.iter().position(|f| f.name == film.name) {
                self.films.remove(idx);
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
