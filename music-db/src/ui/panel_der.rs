use gtk4::gio;
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, ColumnView, ColumnViewColumn, Label, ListItem, Orientation, Revealer,
    RevealerTransitionType, ScrolledWindow, SignalListItemFactory, Spinner, Stack,
    StackTransitionType,
};
use std::rc::Rc;

use super::dialogo_minado;
use super::dialogo_performer;
use super::rola_object::RolaObject;
use super::window::{AppState, SongMeta};
use crate::dao::dao;

pub fn build_right_panel(state: &Rc<AppState>) -> GtkBox {
    let panel = GtkBox::new(Orientation::Vertical, 0);

    // Header?? no se como se llama en espanol
    let header = GtkBox::new(Orientation::Horizontal, 8);
    header.set_margin_top(8);
    header.set_margin_bottom(8);
    header.set_margin_start(12);
    header.set_margin_end(12);

    let status_label = Label::new(Some("music.db"));
    status_label.add_css_class("title-4");
    status_label.set_hexpand(true);
    status_label.set_halign(gtk4::Align::Start);
    header.append(&status_label);

    let btn_edit_performer = Button::with_label("Editar performer");
    btn_edit_performer.set_sensitive(false);
    header.append(&btn_edit_performer);

    let btn_mine = Button::with_label("Minar carpeta");
    btn_mine.add_css_class("suggested-action");
    header.append(&btn_mine);

    panel.append(&header);

    // Minado ruedita
    let minado = Revealer::new();
    minado.set_transition_type(RevealerTransitionType::SlideDown);
    let caja_mina = GtkBox::new(Orientation::Horizontal, 8);
    caja_mina.set_margin_start(12);
    caja_mina.set_margin_end(12);
    caja_mina.set_margin_bottom(8);
    let mining_spinner = Spinner::new();
    let mining_label = Label::new(Some("Minando..."));
    caja_mina.append(&mining_spinner);
    caja_mina.append(&mining_label);
    minado.set_child(Some(&caja_mina));
    panel.append(&minado);

    // Se logro
    let si_se_logro = Revealer::new();
    si_se_logro.set_transition_type(RevealerTransitionType::SlideDown);
    let mostrar_si = Label::new(Some("✓ Minado completo"));
    mostrar_si.set_margin_start(12);
    mostrar_si.set_margin_bottom(8);
    mostrar_si.add_css_class("success");
    si_se_logro.set_child(Some(&mostrar_si));
    panel.append(&si_se_logro);

    // Guardar
    let store = gio::ListStore::new::<RolaObject>();

    // Cargar las que ya estan
    if let Ok(rolas) = dao::obtener_rolas() {
        for rola in &rolas {
            store.append(&RolaObject::new(rola));
        }
    }
    let stack = Stack::new();
    stack.set_transition_type(StackTransitionType::Crossfade);
    stack.set_vexpand(true);

    // Si no hay nada
    let caja_vacia = GtkBox::new(Orientation::Vertical, 12);
    caja_vacia.set_valign(gtk4::Align::Center);
    caja_vacia.set_halign(gtk4::Align::Center);
    let etiqueta_vacia = Label::new(Some("Base de datos vacía"));
    etiqueta_vacia.add_css_class("title-2");
    etiqueta_vacia.add_css_class("dim-label");
    let btn_vacio = Button::with_label("Minar carpeta...");
    btn_vacio.add_css_class("suggested-action");
    btn_vacio.add_css_class("pill");
    caja_vacia.append(&etiqueta_vacia);
    caja_vacia.append(&btn_vacio);
    stack.add_named(&caja_vacia, Some("empty"));

    // Tablas
    let selection = gtk4::SingleSelection::new(Some(store.clone()));
    let column_view = ColumnView::new(Some(selection.clone()));
    column_view.add_css_class("data-table");
    column_view.set_show_column_separators(true);

    // Columnas
    let columnas: Vec<(&str, &str, bool)> = vec![
        ("Track", "track", false),
        ("Título", "title", true),
        ("Álbum", "album", true),
        ("Performer", "performer", true),
        ("Género", "genre", false),
        ("Año", "year", false),
    ];

    for (col_title, prop, expand) in columnas {
        let factoria = SignalListItemFactory::new();
        let prop = prop.to_string();

        factoria.connect_setup(|_, item| {
            let item = item.downcast_ref::<ListItem>().unwrap();
            let etiqueta = Label::new(None);
            etiqueta.set_halign(gtk4::Align::Start);
            etiqueta.set_ellipsize(gtk4::pango::EllipsizeMode::End);
            item.set_child(Some(&etiqueta));
        });

        let prop_bind = prop.clone();
        factoria.connect_bind(move |_, item| {
            let item = item.downcast_ref::<ListItem>().unwrap();
            let rola = item.item().and_downcast::<RolaObject>().unwrap();
            let etiqueta = item.child().and_downcast::<Label>().unwrap();

            if prop_bind == "track" {
                etiqueta.set_text(&rola.track().to_string());
            } else if prop_bind == "year" {
                etiqueta.set_text(&rola.year().to_string());
            } else {
                let valor: String = rola.property(&prop_bind);
                etiqueta.set_text(&valor);
            }
        });

        let columna = ColumnViewColumn::new(Some(col_title), Some(factoria));
        columna.set_expand(expand);
        columna.set_resizable(true);
        if !expand {
            if col_title == "Track" {
                columna.set_fixed_width(50);
            } else {
                columna.set_fixed_width(120);
            }
        }
        column_view.append_column(&columna);
    }

    let scroleado = ScrolledWindow::new();
    scroleado.set_child(Some(&column_view));
    scroleado.set_vexpand(true);
    stack.add_named(&scroleado, Some("table"));

    if store.n_items() > 0 {
        stack.set_visible_child_name("table");
        status_label.set_text(&format!("{} canciones", store.n_items()));
    } else {
        stack.set_visible_child_name("empty");
    }

    panel.append(&stack);

    // Activar boton editar cuando hay seleccion
    let btn_edit_ref = btn_edit_performer.clone();
    let sel_ref = selection.clone();
    selection.connect_selected_notify(move |sel| {
        btn_edit_ref.set_sensitive(sel.selected() != gtk4::INVALID_LIST_POSITION);
    });

    // Click en editar performer
    let sel_edit = sel_ref.clone();
    let store_edit = store.clone();
    btn_edit_performer.connect_clicked(move |btn| {
        let pos = sel_edit.selected();
        if pos == gtk4::INVALID_LIST_POSITION {
            return;
        }
        if let Some(item) = store_edit.item(pos) {
            let rola = item.downcast_ref::<RolaObject>().unwrap();
            let id_rola = rola.id_rola();
            let window = btn.root().and_downcast::<gtk4::Window>().unwrap();
            dialogo_performer::mostrar_dialogo_performer(&window, id_rola, || {});
        }
    });

    let store_rc = Rc::new(store.clone());
    let revelador_minado = Rc::new(minado.clone());
    let revelador_exito = Rc::new(si_se_logro.clone());
    let ruedita_minado = Rc::new(mining_spinner.clone());
    let etiqueta_estado = Rc::new(status_label.clone());
    let stack_rc = Rc::new(stack.clone());

    // Minado
    let contruir_minado = {
        let store_rc = store_rc.clone();
        let minado_revelado = revelador_minado.clone();
        let exito_revelado = revelador_exito.clone();
        let ruedita_minado = ruedita_minado.clone();
        let estado_etiquta = etiqueta_estado.clone();
        let stack_rc = stack_rc.clone();
        let state = state.clone();

        move || {
            let store_rc = store_rc.clone();
            let mining_revealer_rc = minado_revelado.clone();
            let success_revealer_rc = exito_revelado.clone();
            let mining_spinner_rc = ruedita_minado.clone();
            let status_label_rc = estado_etiquta.clone();
            let stack_rc = stack_rc.clone();
            let state = state.clone();

            Box::new(move |ruta: String| {
                super::minado::empezar_minar(
                    &ruta,
                    &store_rc,
                    &mining_revealer_rc,
                    &success_revealer_rc,
                    &mining_spinner_rc,
                    &status_label_rc,
                    &stack_rc,
                    &state,
                );
            }) as Box<dyn Fn(String)>
        }
    };

    // "Minar carpeta" boton
    let build_cb1 = contruir_minado.clone();
    btn_mine.connect_clicked(move |btn| {
        let cb = build_cb1();
        let window = btn.root().and_downcast::<gtk4::Window>().unwrap();
        dialogo_minado::show_mining_dialog(&window, move |ruta| cb(ruta));
    });

    // Estado vacio boton
    let build_cb2 = contruir_minado;
    btn_vacio.connect_clicked(move |btn| {
        let cb = build_cb2();
        let window = btn.root().and_downcast::<gtk4::Window>().unwrap();
        dialogo_minado::show_mining_dialog(&window, move |ruta| cb(ruta));
    });

    // Doble click para que hacer play
    let state_click = state.clone();
    let store_click = store.clone();
    column_view.connect_activate(move |_cv, pos| {
        if let Some(item) = store_click.item(pos) {
            let rola = item.downcast_ref::<RolaObject>().unwrap();
            let rola_path: String = rola.path();

            let mut paths = Vec::new();
            let mut metas = Vec::new();
            for i in 0..store_click.n_items() {
                if let Some(obj) = store_click.item(i) {
                    let r = obj.downcast_ref::<RolaObject>().unwrap();
                    paths.push(r.path());
                    metas.push(SongMeta {
                        title: r.title(),
                        artist: r.performer(),
                        album: r.album(),
                        path: r.path(),
                    });
                }
            }
            state_click.player.set_playlist(paths);
            *state_click.playlist_meta.borrow_mut() = metas;
            state_click.player.play_index(pos as usize);
            *state_click.is_playing.borrow_mut() = true;
            state_click.btn_play.set_icon_name("media-playback-pause-symbolic");

            // Actualizar info a la cancion que este sonando
            *state_click.now_title.borrow_mut() = rola.title();
            *state_click.now_artist.borrow_mut() = rola.performer();
            *state_click.now_album.borrow_mut() = rola.album();
            *state_click.now_art_path.borrow_mut() =
                super::art_album::extraer_y_guardar(&rola_path);
        }
    });

    panel
}
