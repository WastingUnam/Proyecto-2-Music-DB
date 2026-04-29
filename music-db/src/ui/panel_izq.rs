use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, Image, Label, Orientation, Picture, Scale, Stack,
};
use gtk4::glib;
use std::cell::Cell;
use std::rc::Rc;

use super::window::AppState;

pub fn contruye_panel_izq(state: &Rc<AppState>) -> GtkBox {
    let panel = GtkBox::new(Orientation::Vertical, 6);
    panel.set_margin_top(10);
    panel.set_margin_bottom(10);
    panel.set_margin_start(10);
    panel.set_margin_end(10);

    // Album art
    let art_stack = Stack::new();
    let placeholder_img = Image::from_icon_name("audio-x-generic-symbolic");
    placeholder_img.set_pixel_size(250);
    let img_art = Picture::new();
    img_art.set_can_shrink(true);
    img_art.set_size_request(250, 320);
    img_art.set_vexpand(true);
    art_stack.add_named(&placeholder_img, Some("placeholder"));
    art_stack.add_named(&img_art, Some("picture"));
    art_stack.set_visible_child_name("placeholder");
    art_stack.set_vexpand(true);
    panel.append(&art_stack);

    // Empujar los controles hasta abajo
    let spacer = GtkBox::new(Orientation::Vertical, 0);
    spacer.set_vexpand(true);
    panel.append(&spacer);

    // Titulo cancion
    let etiqueta_cancion = Label::new(Some("Sin reproducción"));
    etiqueta_cancion.add_css_class("title-3");
    etiqueta_cancion.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    etiqueta_cancion.set_max_width_chars(25);
    panel.append(&etiqueta_cancion);

    // Artista - Album
    let etiqueta_artista = Label::new(Some("Selecciona una canción"));
    etiqueta_artista.add_css_class("dim-label");
    etiqueta_artista.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    etiqueta_artista.set_max_width_chars(25);
    panel.append(&etiqueta_artista);

    // Barra de progreso
    let progreso = Scale::with_range(Orientation::Horizontal, 0.0, 1.0, 0.01);
    progreso.set_draw_value(false);
    progreso.set_margin_top(8);
    panel.append(&progreso);

    // Time stamps
    let caja_tiempo = GtkBox::new(Orientation::Horizontal, 0);
    let tiempo_actual = Label::new(Some("0:00"));
    tiempo_actual.add_css_class("dim-label");
    tiempo_actual.add_css_class("caption");
    let tiempo_total = Label::new(Some("0:00"));
    tiempo_total.add_css_class("dim-label");
    tiempo_total.add_css_class("caption");
    caja_tiempo.append(&tiempo_actual);
    let time_spacer = GtkBox::new(Orientation::Horizontal, 0);
    time_spacer.set_hexpand(true);
    caja_tiempo.append(&time_spacer);
    caja_tiempo.append(&tiempo_total);
    panel.append(&caja_tiempo);

    // Controles
    let controls = GtkBox::new(Orientation::Horizontal, 12);
    controls.set_halign(Align::Center);
    controls.set_margin_top(8);

    let btn_prev = Button::from_icon_name("media-skip-backward-symbolic");
    let btn_play = state.btn_play.clone();
    let btn_next = Button::from_icon_name("media-skip-forward-symbolic");

    controls.append(&btn_prev);
    controls.append(&btn_play);
    controls.append(&btn_next);
    panel.append(&controls);

    // Volumen
    let vol_box = GtkBox::new(Orientation::Horizontal, 4);
    vol_box.set_margin_top(12);
    let vol_icon = Image::from_icon_name("audio-volume-medium-symbolic");
    let vol_scale = Scale::with_range(Orientation::Horizontal, 0.0, 1.0, 0.05);
    vol_scale.set_value(0.3);
    vol_scale.set_draw_value(false);
    vol_scale.set_hexpand(true);
    vol_box.append(&vol_icon);
    vol_box.append(&vol_scale);
    panel.append(&vol_box);

    // Control del volumen
    let state_vol = state.clone();
    vol_scale.connect_value_changed(move |scale| {
        state_vol.player.set_volume(scale.value());
    });

    // Volumen inicial
    state.player.set_volume(0.1);

    // Play/pausa boton
    let state_play = state.clone();
    let ref_play = btn_play.clone();
    btn_play.connect_clicked(move |_| {
        let mut is_playing = state_play.is_playing.borrow_mut();
        if *is_playing {
            state_play.player.pause();
            ref_play.set_icon_name("media-playback-start-symbolic");
            *is_playing = false;
        } else {
            state_play.player.resume();
            ref_play.set_icon_name("media-playback-pause-symbolic");
            *is_playing = true;
        }
    });

    // Anterior/siguiente
    let state_prev = state.clone();
    btn_prev.connect_clicked(move |_| {
        state_prev.player.prev();
        update_now_playing(&state_prev);
    });
    let state_next = state.clone();
    btn_next.connect_clicked(move |_| {
        state_next.player.next();
        update_now_playing(&state_next);
    });

    // Flag para evitar que el timer dispare seeks (causa crujidos)
    let updating_progress = Rc::new(Cell::new(false));

    let state_seek = state.clone();
    let updating_seek = updating_progress.clone();
    progreso.connect_value_changed(move |scale| {
        if updating_seek.get() {
            return;
        }
        if let Some(dur) = state_seek.player.duration() {
            let pos_ns = (scale.value() * dur.nseconds() as f64) as u64;
            state_seek.player.seek(gstreamer::ClockTime::from_nseconds(pos_ns));
        }
    });

    // Timer de actualización
    let state_timer = state.clone();
    let art_stack_ref = art_stack.clone();
    let art_picture_ref = img_art.clone();
    let title_ref = etiqueta_cancion.clone();
    let artist_ref = etiqueta_artista.clone();
    let updating_timer = updating_progress.clone();

    let mut current_art_path: Option<String> = None;

    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        let pos = state_timer.player.position();
        let dur = state_timer.player.duration();

        if let (Some(pos), Some(dur)) = (pos, dur) {
            let dur_ns = dur.nseconds();
            if dur_ns > 0 {
                let fraction = pos.nseconds() as f64 / dur_ns as f64;
                updating_timer.set(true);
                progreso.set_value(fraction);
                updating_timer.set(false);
            }
            tiempo_actual.set_text(&format_time(pos.seconds()));
            tiempo_total.set_text(&format_time(dur.seconds()));
        }

        // Titulo
        let now_t = state_timer.now_title.borrow();
        if title_ref.text().as_str() != now_t.as_str() {
            title_ref.set_text(&now_t);
        }
        drop(now_t);

        // Artista - Album
        let now_a = state_timer.now_artist.borrow();
        let now_alb = state_timer.now_album.borrow();
        let display = if now_alb.is_empty() {
            now_a.to_string()
        } else {
            format!("{} - {}", &*now_a, &*now_alb)
        };
        if artist_ref.text().as_str() != display.as_str() {
            artist_ref.set_text(&display);
        }
        drop(now_a);
        drop(now_alb);

        // Cover del album
        let now_art = state_timer.now_art_path.borrow();
        if *now_art != current_art_path {
            current_art_path = now_art.clone();
            match current_art_path.as_deref() {
                Some(path) => {
                    art_picture_ref.set_filename(Some(path));
                    art_stack_ref.set_visible_child_name("picture");
                }
                None => {
                    art_stack_ref.set_visible_child_name("placeholder");
                }
            }
        }

        glib::ControlFlow::Continue
    });

    panel
}

fn update_now_playing(state: &Rc<AppState>) {
    if let Some(idx) = state.player.current_index() {
        let meta = state.playlist_meta.borrow();
        if let Some(song) = meta.get(idx) {
            *state.now_title.borrow_mut() = song.title.clone();
            *state.now_artist.borrow_mut() = song.artist.clone();
            *state.now_album.borrow_mut() = song.album.clone();
            *state.now_art_path.borrow_mut() =
                super::art_album::extraer_y_guardar(&song.path);
        }
    }
}

fn format_time(seconds: u64) -> String {
    let mins = seconds / 60;
    let secs = seconds % 60;
    format!("{}:{:02}", mins, secs)
}
