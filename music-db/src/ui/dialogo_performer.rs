use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, DropDown, Entry, Label, Orientation,
    Revealer, RevealerTransitionType, Window,
};
use gtk4::StringList;

use crate::dao::dao;

pub fn mostrar_dialogo_performer(parent: &Window, id_rola: i64, on_save: impl Fn() + 'static) {
    let id_performer = match dao::obtener_id_performer_de_rola(id_rola) {
        Ok(id) => id,
        Err(_) => return,
    };

    let nombre = dao::obtener_nombre_performer(id_performer).unwrap_or_default();
    let tipo_actual = dao::obtener_tipo_performer(id_performer).unwrap_or(2);
    let persona_actual = dao::obtener_persona(id_performer).ok().flatten();
    let grupo_actual = dao::obtener_grupo(id_performer).ok().flatten();

    let dialogo = Window::builder()
        .title(format!("Editar performer: {}", nombre))
        .modal(true)
        .transient_for(parent)
        .default_width(400)
        .default_height(350)
        .resizable(false)
        .build();

    let vbox = GtkBox::new(Orientation::Vertical, 12);
    vbox.set_margin_top(20);
    vbox.set_margin_bottom(20);
    vbox.set_margin_start(20);
    vbox.set_margin_end(20);

    // Tipo dropdown
    let lbl_tipo = Label::new(Some("Tipo de performer:"));
    lbl_tipo.set_halign(Align::Start);
    vbox.append(&lbl_tipo);

    let opciones = StringList::new(&["Persona", "Grupo"]);
    let dropdown = DropDown::new(Some(opciones), gtk4::Expression::NONE);
    let indice_inicial = match tipo_actual {
        0 => 0, // Person
        1 => 1, // Group
        _ => 0, // Unknown -> default a Persona
    };
    dropdown.set_selected(indice_inicial);
    vbox.append(&dropdown);

    // Seccion persona con Revealer
    let revealer = Revealer::new();
    revealer.set_transition_type(RevealerTransitionType::SlideDown);

    let seccion_persona = GtkBox::new(Orientation::Vertical, 8);

    let lbl_stage = Label::new(Some("Nombre artístico:"));
    lbl_stage.set_halign(Align::Start);
    seccion_persona.append(&lbl_stage);
    let entry_stage = Entry::new();
    entry_stage.set_placeholder_text(Some("Nombre artístico"));
    seccion_persona.append(&entry_stage);

    let lbl_real = Label::new(Some("Nombre real:"));
    lbl_real.set_halign(Align::Start);
    seccion_persona.append(&lbl_real);
    let entry_real = Entry::new();
    entry_real.set_placeholder_text(Some("Nombre real"));
    seccion_persona.append(&entry_real);

    let lbl_birth = Label::new(Some("Fecha de nacimiento:"));
    lbl_birth.set_halign(Align::Start);
    seccion_persona.append(&lbl_birth);
    let entry_birth = Entry::new();
    entry_birth.set_placeholder_text(Some("Fecha"));
    seccion_persona.append(&entry_birth);

    let lbl_death = Label::new(Some("Fecha de defunción:"));
    lbl_death.set_halign(Align::Start);
    seccion_persona.append(&lbl_death);
    let entry_death = Entry::new();
    entry_death.set_placeholder_text(Some("Fecha"));
    seccion_persona.append(&entry_death);

    // Prellenar si ya hay datos de persona
    if let Some((stage, real, birth, death)) = persona_actual {
        entry_stage.set_text(&stage);
        entry_real.set_text(&real);
        entry_birth.set_text(&birth);
        entry_death.set_text(&death);
    }

    revealer.set_child(Some(&seccion_persona));
    revealer.set_reveal_child(indice_inicial == 0);
    vbox.append(&revealer);

    // Seccion grupo con Revealer
    let revealer_grupo = Revealer::new();
    revealer_grupo.set_transition_type(RevealerTransitionType::SlideDown);

    let seccion_grupo = GtkBox::new(Orientation::Vertical, 8);

    let lbl_start = Label::new(Some("Fecha de inicio:"));
    lbl_start.set_halign(Align::Start);
    seccion_grupo.append(&lbl_start);
    let entry_start = Entry::new();
    entry_start.set_placeholder_text(Some("Fecha"));
    seccion_grupo.append(&entry_start);

    let lbl_end = Label::new(Some("Fecha de fin:"));
    lbl_end.set_halign(Align::Start);
    seccion_grupo.append(&lbl_end);
    let entry_end = Entry::new();
    entry_end.set_placeholder_text(Some("Fecha"));
    seccion_grupo.append(&entry_end);

    // Prellenar si ya hay datos de grupo
    if let Some((_name, start, end)) = grupo_actual {
        entry_start.set_text(&start);
        entry_end.set_text(&end);
    }

    revealer_grupo.set_child(Some(&seccion_grupo));
    revealer_grupo.set_reveal_child(indice_inicial == 1);
    vbox.append(&revealer_grupo);

    // Mostrar/ocultar al cambiar dropdown
    let revealer_clone = revealer.clone();
    let revealer_grupo_clone = revealer_grupo.clone();
    dropdown.connect_selected_notify(move |dd| {
        revealer_clone.set_reveal_child(dd.selected() == 0);
        revealer_grupo_clone.set_reveal_child(dd.selected() == 1);
    });

    // Botones
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

    // Cancelar
    let dialogo_cancel = dialogo.clone();
    btn_cancelar.connect_clicked(move |_| {
        dialogo_cancel.close();
    });

    // Guardar
    let dialogo_save = dialogo.clone();
    btn_guardar.connect_clicked(move |_| {
        let seleccion = dropdown.selected();
        let resultado = if seleccion == 0 {
            // Persona
            dao::actualizar_performer_persona(
                id_performer,
                &entry_stage.text(),
                &entry_real.text(),
                &entry_birth.text(),
                &entry_death.text(),
            )
        } else {
            // Grupo
            dao::actualizar_performer_grupo(
                id_performer,
                &entry_start.text(),
                &entry_end.text(),
            )
        };

        match resultado {
            Ok(()) => {
                on_save();
                dialogo_save.close();
            }
            Err(e) => {
                eprintln!("Error al guardar performer: {}", e);
            }
        }
    });

    dialogo.present();
}
