use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, CheckButton, Entry, Orientation,
    ScrolledWindow, Window,
};

use crate::dao::dao;

pub fn mostrar_dialogo_miembros(parent: &Window, id_group: i64, nombre_grupo: &str) {
    let personas = match dao::obtener_todas_personas() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error al obtener personas: {}", e);
            return;
        }
    };

    let miembros_actuales = dao::obtener_miembros_grupo(id_group).unwrap_or_default();

    let dialogo = Window::builder()
        .title(format!("Miembros de: {}", nombre_grupo))
        .modal(true)
        .transient_for(parent)
        .default_width(350)
        .default_height(450)
        .resizable(false)
        .build();

    let vbox = GtkBox::new(Orientation::Vertical, 8);
    vbox.set_margin_top(12);
    vbox.set_margin_bottom(12);
    vbox.set_margin_start(12);
    vbox.set_margin_end(12);

    let lista = GtkBox::new(Orientation::Vertical, 4);

    let checks: Rc<RefCell<Vec<(i64, CheckButton)>>> = Rc::new(RefCell::new(Vec::new()));

    for (id, nombre) in &personas {
        let check = CheckButton::with_label(nombre);
        if miembros_actuales.contains(id) {
            check.set_active(true);
        }
        lista.append(&check);
        checks.borrow_mut().push((*id, check));
    }

    let scroll = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .child(&lista)
        .build();
    vbox.append(&scroll);

    // Para agregar persona
    let caja_nueva = GtkBox::new(Orientation::Horizontal, 8);
    caja_nueva.set_margin_top(8);

    let entry_nombre = Entry::new();
    entry_nombre.set_placeholder_text(Some("Nombre de nueva persona"));
    entry_nombre.set_hexpand(true);
    caja_nueva.append(&entry_nombre);

    let btn_agregar = Button::with_label("Agregar");
    caja_nueva.append(&btn_agregar);
    vbox.append(&caja_nueva);

    let lista_ref = lista.clone();
    let checks_ref = checks.clone();
    btn_agregar.connect_clicked(move |_| {
        let nombre = entry_nombre.text();
        let nombre = nombre.trim();
        if nombre.is_empty() {
            return;
        }

        match dao::crear_persona(nombre) {
            Ok(id) => {
                let check = CheckButton::with_label(nombre);
                check.set_active(true);
                lista_ref.append(&check);
                checks_ref.borrow_mut().push((id, check));
                entry_nombre.set_text("");
            }
            Err(e) => eprintln!("Error al crear persona: {}", e),
        }
    });

    // Guardar / cancelar
    let caja_botones = GtkBox::new(Orientation::Horizontal, 8);
    caja_botones.set_halign(Align::End);
    caja_botones.set_margin_top(12);

    let btn_cancelar = Button::with_label("Cancelar");
    let btn_guardar = Button::with_label("Guardar");
    btn_guardar.add_css_class("suggested-action");

    caja_botones.append(&btn_cancelar);
    caja_botones.append(&btn_guardar);
    vbox.append(&caja_botones);

    dialogo.set_child(Some(&vbox));

    let dialogo_cancel = dialogo.clone();
    btn_cancelar.connect_clicked(move |_| {
        dialogo_cancel.close();
    });

    let dialogo_save = dialogo.clone();
    btn_guardar.connect_clicked(move |_| {
        let seleccionados: Vec<i64> = checks
            .borrow()
            .iter()
            .filter(|(_, check)| check.is_active())
            .map(|(id, _)| *id)
            .collect();

        match dao::actualizar_miembros_grupo(id_group, &seleccionados) {
            Ok(()) => dialogo_save.close(),
            Err(e) => eprintln!("Error al guardar miembros: {}", e),
        }
    });

    dialogo.present();
}
