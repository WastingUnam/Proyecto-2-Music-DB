use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

use crate::playback::player::Player;
use super::panel_izq;
use super::right_panel;

pub struct SongMeta {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub path: String,
}

pub struct AppState {
    pub player: Player,
    pub is_playing: RefCell<bool>,
    pub now_title: RefCell<String>,
    pub now_artist: RefCell<String>,
    pub now_album: RefCell<String>,
    pub now_art_path: RefCell<Option<String>>,
    pub playlist_meta: RefCell<Vec<SongMeta>>,
}

pub fn build_window(app: &Application) {
    let state = Rc::new(AppState {
        player: Player::new(),
        is_playing: RefCell::new(false),
        now_title: RefCell::new(String::from("Sin reproducción")),
        now_artist: RefCell::new(String::from("Selecciona una canción")),
        now_album: RefCell::new(String::new()),
        now_art_path: RefCell::new(None),
        playlist_meta: RefCell::new(Vec::new()),
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("music.db")
        .default_width(1000)
        .default_height(1000)
        .build();

    let paned = gtk4::Paned::builder()
        .orientation(Orientation::Horizontal)
        .position(250) // 1/4 of 1000 default width
        .build();

    let left = panel_izq::contruye_panel_izq(&state);
    let right = right_panel::build_right_panel(&state);

    paned.set_start_child(Some(&left));
    paned.set_end_child(Some(&right));
    paned.set_resize_start_child(false);
    paned.set_shrink_start_child(false);
    paned.set_resize_end_child(true);
    paned.set_shrink_end_child(true);

    window.set_child(Some(&paned));
    window.present();
}
