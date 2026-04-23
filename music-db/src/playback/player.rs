use gstreamer::prelude::*;
use gstreamer::glib;
use gstreamer_player::Player as GstPlayer;
use std::cell::RefCell;

pub struct Player {
    gst_player: GstPlayer,
    playlist: RefCell<Vec<String>>,
    current_index: RefCell<Option<usize>>,
}

impl Player {
    pub fn new() -> Self {
        let gst_player = GstPlayer::new(
            None::<gstreamer_player::PlayerVideoRenderer>,
            None::<gstreamer_player::PlayerSignalDispatcher>,
        );

        // Configurar pipeline como Lollypop: buffers grandes + audio sink personalizado
        let pipeline = gst_player.pipeline();
        pipeline.set_property("buffer-size", 5_i32 << 20); // 5 MB
        pipeline.set_property("buffer-duration", 10_i64 * gstreamer::ClockTime::SECOND.nseconds() as i64);

        // Deshabilitar video (flag bit 0x01 = audio, 0x02 = video, 0x04 = text)
        let flags: glib::Value = pipeline.property("flags");
        if let Ok(flags_int) = flags.get::<u32>() {
            pipeline.set_property("flags", flags_int & !0x02_u32);
        }

        // Audio sink con audioconvert + audioresample para evitar crujidos
        if let Some(audio_sink) = Self::build_audio_sink() {
            pipeline.set_property("audio-sink", &audio_sink);
        }

        Self {
            gst_player,
            playlist: RefCell::new(Vec::new()),
            current_index: RefCell::new(None),
        }
    }

    fn build_audio_sink() -> Option<gstreamer::Element> {
        let bin = gstreamer::Bin::new();
        let convert = gstreamer::ElementFactory::make("audioconvert").build().ok()?;
        let resample = gstreamer::ElementFactory::make("audioresample").build().ok()?;
        let sink = gstreamer::ElementFactory::make("autoaudiosink").build().ok()?;

        bin.add_many([&convert, &resample, &sink]).ok()?;
        gstreamer::Element::link_many([&convert, &resample, &sink]).ok()?;

        let pad = convert.static_pad("sink")?;
        let ghost = gstreamer::GhostPad::with_target(&pad).ok()?;
        bin.add_pad(&ghost).ok()?;

        Some(bin.upcast())
    }

    pub fn play_file(&self, path: &str) {
        let uri = if path.starts_with("file://") {
            path.to_string()
        } else {
            format!("file://{}", path)
        };
        self.gst_player.set_uri(Some(&uri));
        self.gst_player.play();
    }

    pub fn set_playlist(&self, paths: Vec<String>) {
        *self.playlist.borrow_mut() = paths;
    }

    pub fn play_index(&self, index: usize) {
        let playlist = self.playlist.borrow();
        if let Some(path) = playlist.get(index) {
            let path = path.clone();
            drop(playlist);
            *self.current_index.borrow_mut() = Some(index);
            self.play_file(&path);
        }
    }

    pub fn pause(&self) {
        self.gst_player.pause();
    }

    pub fn resume(&self) {
        self.gst_player.play();
    }

    pub fn next(&self) {
        let idx = self.current_index.borrow().unwrap_or(0);
        let len = self.playlist.borrow().len();
        if len > 0 {
            self.play_index((idx + 1) % len);
        }
    }

    pub fn prev(&self) {
        let idx = self.current_index.borrow().unwrap_or(0);
        let len = self.playlist.borrow().len();
        if len > 0 {
            let new_idx = if idx == 0 { len - 1 } else { idx - 1 };
            self.play_index(new_idx);
        }
    }

    pub fn set_volume(&self, volume: f64) {
        self.gst_player.set_volume(volume);
    }

    pub fn position(&self) -> Option<gstreamer::ClockTime> {
        self.gst_player.position()
    }

    pub fn duration(&self) -> Option<gstreamer::ClockTime> {
        self.gst_player.duration()
    }

    pub fn seek(&self, position: gstreamer::ClockTime) {
        self.gst_player.seek(position);
    }

    pub fn current_index(&self) -> Option<usize> {
        *self.current_index.borrow()
    }
}
