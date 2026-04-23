use gtk4::prelude::*;
use gtk4::{gio, Revealer, Stack, Label, Spinner};
use gtk4::glib;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;

use crate::dao::dao;
use crate::minero::minero::mina;
use crate::minero::audio::Cancion;
use super::rola_object::RolaObject;
use super::window::AppState;

/// Empieza a minar y actualiza cuando acabe
pub fn empezar_minar(
    ruta: &str,
    store: &Rc<gio::ListStore>,
    mining_revealer: &Rc<Revealer>,
    success_revealer: &Rc<Revealer>,
    mining_spinner: &Rc<Spinner>,
    status_label: &Rc<Label>,
    stack: &Rc<Stack>,
    _state: &Rc<AppState>,
) {
    let (tx, rx) = mpsc::channel::<Vec<Cancion>>();

    mining_revealer.set_reveal_child(true);
    mining_spinner.set_spinning(true);
    success_revealer.set_reveal_child(false);

    let ruta = ruta.to_string();
    std::thread::spawn(move || {
        let canciones = mina(&ruta);
        let _ = tx.send(canciones);
    });

    let store = store.clone();
    let mining_revealer = mining_revealer.clone();
    let success_revealer = success_revealer.clone();
    let mining_spinner = mining_spinner.clone();
    let status_label = status_label.clone();
    let stack = stack.clone();

    // Abrir conexión a DB una sola vez
    let db_path = dao::db_path();
    let db_exists = db_path.exists();
    let conn: Rc<RefCell<Option<rusqlite::Connection>>> = Rc::new(RefCell::new(None));
    if let Ok(c) = rusqlite::Connection::open(&db_path) {
        if !db_exists {
            dao::iniciar_schema(&c);
        }
        *conn.borrow_mut() = Some(c);
    }

    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        match rx.try_recv() {
            Ok(canciones) => {
                // Insertar todas en DB
                if let Some(ref c) = *conn.borrow() {
                    for cancion in &canciones {
                        let _ = dao::insertar_cancion(c, cancion);
                    }
                }
                for cancion in &canciones {
                    let _ = super::art_album::extraer_y_guardar(&cancion.path);

                    let rola_view = dao::RolaView {
                        id_rola: 0,
                        title: cancion.title.clone(),
                        album: cancion.album.clone(),
                        performer: cancion.artist.clone(),
                        year: cancion.year,
                        genre: cancion.genre.clone(),
                        path: cancion.path.clone(),
                    };
                    store.append(&RolaObject::new(&rola_view));
                }

                // Actualizar UI
                mining_revealer.set_reveal_child(false);
                mining_spinner.set_spinning(false);
                success_revealer.set_reveal_child(true);
                stack.set_visible_child_name("table");
                status_label.set_text(&format!("{} canciones", store.n_items()));

                let sr = success_revealer.clone();
                glib::timeout_add_local_once(
                    std::time::Duration::from_secs(3),
                    move || {
                        sr.set_reveal_child(false);
                    },
                );

                glib::ControlFlow::Break
            }
            Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
            Err(mpsc::TryRecvError::Disconnected) => {
                mining_revealer.set_reveal_child(false);
                mining_spinner.set_spinning(false);
                glib::ControlFlow::Break
            }
        }
    });
}
