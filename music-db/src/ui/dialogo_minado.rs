use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, Entry, Label, Orientation, Window,};

pub fn show_mining_dialog<F: Fn(String) + 'static>(parent: &Window, on_mine: F) {
    let dialogo = Window::builder()
        .title("Minar carpeta")
        .modal(true)
        .transient_for(parent)
        .default_width(450)
        .default_height(180)
        .resizable(false)
        .build();

    let vbox = GtkBox::new(Orientation::Vertical, 12);
    vbox.set_margin_top(20);
    vbox.set_margin_bottom(20);
    vbox.set_margin_start(20);
    vbox.set_margin_end(20);

    let etiqueta = Label::new(Some("Ruta hacia la carpeta con musica:"));
    etiqueta.set_halign(Align::Start);
    vbox.append(&etiqueta);

    // Escribir el path
    let path_box = GtkBox::new(Orientation::Horizontal, 8);
    let entrada = Entry::new();
    entrada.set_hexpand(true);
    entrada.set_placeholder_text(Some("/home/usuario/Music"));

    // Default a ~/Music
    if let Some(home) = dirs::home_dir() {
        entrada.set_text(&format!("{}/Music", home.display()));
    }

    // Boton para buscar
    let btn_browse = Button::with_label("...");
    path_box.append(&entrada);
    path_box.append(&btn_browse);
    vbox.append(&path_box);

    // Buscando con el de archivos
    let browse = entrada.clone();
    let ref_dialogo = dialogo.clone();
    btn_browse.connect_clicked(move |_| {
        let chooser = gtk4::FileChooserNative::new(
            Some("Seleccionar carpeta"),
            Some(&ref_dialogo),
            gtk4::FileChooserAction::SelectFolder,
            Some("Seleccionar"),
            Some("Cancelar"),
        );
        let entry = browse.clone();
        chooser.connect_response(move |chooser, response| {
            if response == gtk4::ResponseType::Accept {
                if let Some(file) = chooser.file() {
                    if let Some(path) = file.path() {
                        entry.set_text(&path.to_string_lossy());
                    }
                }
            }
        });
        chooser.show();
    });

    // Botones
    let caja_botones = GtkBox::new(Orientation::Horizontal, 8);
    caja_botones.set_halign(Align::End);
    caja_botones.set_margin_top(8);

    let btn_cancelar = Button::with_label("Cancelar");
    let btn_iniciar = Button::with_label("Empezar a minar");
    btn_iniciar.add_css_class("suggested-action");

    caja_botones.append(&btn_cancelar);
    caja_botones.append(&btn_iniciar);
    vbox.append(&caja_botones);

    dialogo.set_child(Some(&vbox));

    // Cancel
    let dialogo_cancelar = dialogo.clone();
    btn_cancelar.connect_clicked(move |_| {
        dialogo_cancelar.close();
    });

    // Start mining
    let dialogo_iniciar = dialogo.clone();
    let entrada_iniciar = entrada.clone();
    btn_iniciar.connect_clicked(move |_| {
        let ruta = entrada_iniciar.text().to_string();
        if !ruta.is_empty() {
            dialogo_iniciar.close();
            on_mine(ruta);
        }
    });

    dialogo.present();
}
