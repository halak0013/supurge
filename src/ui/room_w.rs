use eframe::egui::{self, Color32, RichText, Stroke};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Room {
    pub name: String,
    pub dirty_state: bool,
}

impl Room {
    pub fn new(name: String) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            name,
            dirty_state: rng.gen(),
        }
    }
    pub fn random_dirty(&mut self) {
        let mut rng = rand::thread_rng();
        self.dirty_state = rng.gen();
    }
    pub fn clean_room(&mut self) -> String {
        self.dirty_state = false;
        format!(" cleaned {}", self.name)
    }
}

impl std::fmt::Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} is {} ",
            self.name,
            if self.dirty_state { "dirty" } else { "clean" },
        )
    }
}

pub fn get_all_room_state(rooms: &[Room]) -> String {
    let mut res = String::new();
    for (i, r) in rooms.iter().enumerate() {
        if i != rooms.len() - 1 {
            res += &format!("{}, ", r.to_string());
        } else {
            res += &format!("{}", r.to_string());
        }
    }
    res
}
pub fn get_all_room_clean_dirty_count(rooms: &[Room]) -> (usize, usize) {
    /* rooms.iter().for_each(|room| {
        if room.dirty_state {
            dirty_rooms += 1;
        } else {
            clean_rooms += 1;
        }
    }); */
    let (clean_rooms, dirty_rooms): (usize, usize) = rooms
        .iter()
        .map(|room| if room.dirty_state { 0 } else { 1 })
        .fold((0, 0), |(clean, dirty), state| {
            (clean + state, dirty + (1 - state))
        });
    (clean_rooms, dirty_rooms)
}

pub fn room(ui: &mut egui::Ui, room: &Room, is_selected: bool) {
    let color = if room.dirty_state {
        Color32::from_rgb(200, 0, 0)
    } else {
        Color32::from_rgb(0, 200, 0)
    };
    let stroke = if is_selected {
        Stroke::new(4., Color32::from_rgb(10, 10, 10))
    } else {
        Stroke::NONE
    };
    egui::Frame::default()
        .fill(color)
        .stroke(stroke)
        .inner_margin(10.)
        .rounding(10.)
        .show(ui, |ui| {
            ui.label(RichText::new(format!(" {} ", &room.name)).color(Color32::BLACK));
        });
    ui.add_space(20.);
}
