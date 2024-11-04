pub mod ui;

use eframe::egui::{self, DragValue, ScrollArea};

use egui_plot::{PlotPoint, PlotPoints};
// A - B
/// oda struct'ı
/// oda içinde isim ve temizlik durumu
/// her adımda çalışacak kontrol fonksiyon sonrası rastgele kirletme fonksiyonu
// odaların tutulduğu liste
// geçmişin tutulduğu liste
use ui::plot::CustomPlotUi;
use ui::room_w::{get_all_room_clean_dirty_count, get_all_room_state, room, Room};

#[derive(Debug)]
struct AppMemory {
    current_index: usize,
    history: Vec<History>,
    rooms: Vec<Room>,
    iteration_count: u32,
}

impl Default for AppMemory {
    fn default() -> Self {
        let ra = Room::new("A".to_string());
        let rb = Room::new("B".to_string());
        let rc = Room::new("C".to_string());
        Self {
            current_index: Default::default(),
            history: Default::default(),
            iteration_count: Default::default(),
            rooms: vec![ra, rb, rc],
        }
    }
}

#[derive(Debug, Default)]
struct History {
    state: String,
    clean_count: usize,
    dirty_count: usize,
    clean_count_sum: usize,
    dirty_count_sum: usize,
}

fn main() {
    let app_mem = std::rc::Rc::new(std::cell::RefCell::new(AppMemory::default()));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 500.0]),
        ..Default::default()
    };

    eframe::run_simple_native("Süpürge", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (i, ele) in app_mem.borrow().rooms.iter().enumerate() {
                        room(ui, ele, i == app_mem.borrow().current_index);
                    }
                });
            });
            ui.add_space(10.);
            ui.horizontal(|ui| {
                // room insert delete buttons
                if ui.button("Insert Room").clicked() {
                    let ch = (65 + app_mem.borrow().rooms.len()) as u8 as char;
                    app_mem.borrow_mut().rooms.push(Room::new(ch.into()));
                }
                if ui.button("Delete Room").clicked() {
                    if app_mem.borrow().rooms.len() > 1 {
                        app_mem.borrow_mut().rooms.pop();
                    }
                }
            });
            ui.add_space(10.);
            ui.horizontal(|ui| {
                if ui.button("Next iteration").clicked() {
                    next_iteration(&mut app_mem.borrow_mut());
                    if app_mem.borrow().current_index == app_mem.borrow().rooms.len() - 1 {
                        app_mem.borrow_mut().current_index = 0;
                    } else {
                        app_mem.borrow_mut().current_index += 1;
                    }
                }
                ui.label("---->");
                ui.add(DragValue::new(&mut app_mem.borrow_mut().iteration_count));

                if ui.button("Skip steps").clicked() {
                    let iter_count = app_mem.borrow().iteration_count;
                    for _ in 0..iter_count {
                        next_iteration(&mut app_mem.borrow_mut());
                        if app_mem.borrow().current_index == app_mem.borrow().rooms.len() - 1 {
                            app_mem.borrow_mut().current_index = 0;
                        } else {
                            app_mem.borrow_mut().current_index += 1;
                        }
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    let get_clean_percent = |c: f64, d: f64| -> f64 { (c / (c + d)) * 100. };
                    let points: PlotPoints = PlotPoints::from_iter(
                        app_mem.borrow().history.iter().enumerate().map(|(i, h)| {
                            [
                                i as f64,
                                get_clean_percent(h.clean_count as f64, h.dirty_count as f64),
                            ]
                        }),
                    );
                    let label_formatter = |_s: &str, val: &PlotPoint| {
                        format!("Clean rate: {:.2}\n Step: {:.0}", val.y, val.x)
                    };
                    let plot_ui = CustomPlotUi::new(
                        "Step".into(),
                        "Value".into(),
                        points,
                        Box::new(label_formatter),
                        "Cleaning rate by step".into(),
                    );
                    let points_: PlotPoints = PlotPoints::from_iter(
                        app_mem.borrow().history.iter().enumerate().map(|(i, h)| {
                            [
                                i as f64,
                                get_clean_percent(
                                    h.clean_count_sum as f64,
                                    h.dirty_count_sum as f64,
                                ),
                            ]
                        }),
                    );
                    let summary_plot_ui = CustomPlotUi::new(
                        "Step".into(),
                        "Percent".into(),
                        points_,
                        Box::new(label_formatter),
                        "Aggregate cleaning rate".into(),
                    );
                    ui.add(plot_ui);
                    ui.add(summary_plot_ui);
                });
                ScrollArea::vertical().max_height(180.).show(ui, |ui| {
                    ui.vertical(|ui| {
                        let mut s = String::new();
                        for (i, h) in app_mem.borrow().history.iter().enumerate() {
                            s += &format!("{}-) {}\n", i, h.state);
                        }
                        ui.label(s);
                    });
                });
            });
        });
    })
    .unwrap();
}

fn next_iteration(app_mem: &mut AppMemory) {
    let mut curr = app_mem.current_index;
    if curr > app_mem.rooms.len() {
        app_mem.current_index = app_mem.rooms.len() - 1;
        curr = app_mem.current_index;
    }
    let cur_state = app_mem.rooms[curr].dirty_state;
    let mut s = format!(
        "{}-> {}",
        app_mem.rooms[curr].name,
        get_all_room_state(&app_mem.rooms)
    );
    if cur_state {
        s += &format!(" | {}", app_mem.rooms[curr].clean_room());
    }
    app_mem.rooms[curr].random_dirty();
    let (clean_count, dirty_count) = get_all_room_clean_dirty_count(&app_mem.rooms);
    let h = &app_mem.history;
    app_mem.history.push(History {
        state: s,
        clean_count,
        dirty_count,
        clean_count_sum: h.last().map_or(0, |last| last.clean_count) + clean_count,
        dirty_count_sum: h.last().map_or(0, |last| last.dirty_count) + dirty_count,
    });
}
