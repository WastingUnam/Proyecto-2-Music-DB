use gtk4::glib;
use gtk4::glib::Properties;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use std::cell::RefCell;

use crate::dao::dao::RolaView;

mod imp {
    use super::*;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::RolaObject)]
    pub struct RolaObject {
        #[property(get, set)]
        id_rola: RefCell<i64>,
        #[property(get, set)]
        title: RefCell<String>,
        #[property(get, set)]
        album: RefCell<String>,
        #[property(get, set)]
        performer: RefCell<String>,
        #[property(get, set)]
        year: RefCell<i32>,
        #[property(get, set)]
        genre: RefCell<String>,
        #[property(get, set)]
        path: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RolaObject {
        const NAME: &'static str = "RolaObject";
        type Type = super::RolaObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for RolaObject {}
}

glib::wrapper! {
    pub struct RolaObject(ObjectSubclass<imp::RolaObject>);
}

impl RolaObject {
    pub fn new(rola: &RolaView) -> Self {
        glib::Object::builder()
            .property("id-rola", rola.id_rola)
            .property("title", &rola.title)
            .property("album", &rola.album)
            .property("performer", &rola.performer)
            .property("year", rola.year)
            .property("genre", &rola.genre)
            .property("path", &rola.path)
            .build()
    }
}
