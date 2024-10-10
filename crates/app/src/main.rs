mod app_state;
mod app_workspace;
mod assets;
// mod audio;
// mod library;
// mod ui;
// mod views;

use anyhow::Result;
use app_state::AppState;
use assets::Assets;
use gpui::prelude::*;
use gpui::*;
use gpui::{actions, App, AppContext, KeyBinding, Menu, MenuItem};
// use library::{Library, Song};
use parking_lot::Mutex;
use rodio::Sink;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fs, future::Future, io::BufReader, sync::Arc, time::Duration};
use ui::input::{Copy, Cut, Paste, Redo, Undo};
// use views::{loading_ui::LoadingUI, main_ui::MainUI, settings_ui::SettingsUI};

// actions!(
//     jukebox,
//     [
//         TogglePlayback,
//         PreviousSong,
//         NextSong,
//         AddTestSongToQueue,
//         Scan,
//         LoadLibrary,
//         TickUpdate,
//     ]
// );

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct GlobalSettings {
//     folder_to_scan: String, // TODO add ability to scan multiple folders
//     library_file: String,   // where the serialized library is saved
//                             // theme: VisualTheme
// }

// impl Default for GlobalSettings {
//     fn default() -> Self {
//         Self {
//             folder_to_scan: String::from("./"),
//             library_file: String::from("library.toml"),
//         }
//     }
// }

// #[derive(Clone, Copy, Debug)]
// struct PlaybackSettings {
//     volume: f32, // lets leave this at 1.0 for now
//     speed: f32,  // lets leave this at 1.0 for now
// }

// impl Default for PlaybackSettings {
//     fn default() -> Self {
//         Self {
//             volume: 1.0,
//             speed: 1.0,
//         }
//     }
// }

// #[derive(Debug, Clone)]
// enum Message {
//     TogglePlayback,
//     PreviousSong,
//     NextSong,
//     AddTestSongToQueue,
//     PickSong(Uuid),
//     Scan,
//     ScanComplete(Result<(), String>),
//     LoadComplete(Result<(), String>),
//     SaveSettings(GlobalSettings),
//     ChangeUI(UIState),
//     TickUpdate,
// }

// #[derive(Debug, Clone)]
// enum UIState {
//     Loading,
//     Main, //current screen
//     Settings,
//     // Artist(id) // not sure how to best implement
//     // Album(id) // not sure how to best implement
//     // Song?(id) // not sure how to best implement
// }

// #[derive(Clone)]
// struct Jukebox {
//     sink: Arc<Mutex<Option<Sink>>>,
//     global_settings: GlobalSettings,
//     playback_settings: PlaybackSettings,
//     ui_state: UIState,
//     music_library: Arc<Mutex<Library>>,
//     playback_queue: Arc<Mutex<VecDeque<(Song, bool)>>>,
//     playback_index: usize,
// }

// impl Default for Jukebox {
//     fn default() -> Self {
//         Self {
//             sink: Arc::new(Mutex::new(None)),
//             global_settings: Self::read_or_create_config(),
//             playback_settings: PlaybackSettings::default(), // TODO fetch
//             ui_state: UIState::Loading,
//             music_library: Arc::new(Mutex::new(Library::new())),
//             playback_queue: Arc::new(Mutex::new(VecDeque::new())),
//             playback_index: 0,
//         }
//     }
// }

// Functionality
// impl Jukebox {
//     // fn init() -> Self {
//     //     Jukebox {
//     //         sink: Arc::new(Mutex::new(None)),
//     //         global_settings: Self::read_or_create_config(),
//     //         playback_settings: PlaybackSettings::default(), // TODO fetch
//     //         ui_state: UIState::Loading,
//     //         music_library: Arc::new(Mutex::new(Library::new())),
//     //         playback_queue: Arc::new(Mutex::new(VecDeque::new())),
//     //         playback_index: 0,
//     //     }
//     // }

//     fn toggle_sink_playback(&mut self) {
//         if self.sink.lock().as_ref().unwrap().is_paused() {
//             self.sink.lock().as_ref().unwrap().play();
//         } else {
//             self.sink.lock().as_ref().unwrap().pause()
//         }
//     }

//     fn reorder_song_in_queue(&self, new_pos_in_queue: usize) -> Result<()> {
//         todo!()
//     }

//     fn add_song_to_queue_end(&self, song: Song) -> Result<()> {
//         self.playback_queue.lock().push_back((song, false));
//         Ok(())
//     }

//     fn add_song_to_queue_start(&self, song: Song) -> Result<()> {
//         self.playback_queue.lock().push_front((song, false));
//         Ok(())
//     }

//     fn play_song_from_queue(&mut self) -> Result<()> {
//         self.replace_sink()?;

//         for (_song, current) in self.playback_queue.lock().iter_mut() {
//             *current = false;
//         }

//         if let Some((song, _is_current)) = self.playback_queue.lock().get_mut(self.playback_index) {
//             *_is_current = true;

//             self.sink
//                 .lock()
//                 .as_ref()
//                 .unwrap()
//                 .append(rodio::Decoder::new(BufReader::new(std::fs::File::open(
//                     &song.file_path,
//                 )?))?);
//             println!(
//                 "added song: {} by {}",
//                 song.title,
//                 song.artists.first().unwrap().name
//             );
//         }

//         Ok(())
//     }

//     fn replace_sink(&mut self) -> Result<()> {
//         self.kill_sink()?;
//         self.sink = Arc::new(Mutex::new(Some(audio::new_sink(self.playback_settings))));
//         println!("sink created");
//         Ok(())
//     }

//     fn kill_sink(&mut self) -> Result<()> {
//         if self.sink.lock().as_ref().is_some() {
//             self.sink = Arc::new(Mutex::new(None));
//             println!("sink killed");
//         }
//         Ok(())
//     }

//     // fn stop_current_playback(&mut self) -> Result<()> {}

//     fn update_time(&mut self) {
//         let time_remaining = self
//             .playback_queue
//             .lock()
//             .get(self.playback_index)
//             .unwrap_or(&(Song::default(), false))
//             .0
//             .duration
//             // .as_secs()
//             - self
//                 .sink
//                 .lock()
//                 .as_ref()
//                 .unwrap_or(&Sink::new_idle().0)
//                 .get_pos();
//         // .as_secs();
//         // println!("song duration remaining: {:?}", time_remaining);
//         if self.sink.lock().is_some() {
//             if !self.sink.lock().as_ref().unwrap().is_paused() && time_remaining <= Duration::ZERO {
//                 self.next_in_queue();
//             }
//         }
//     }

//     fn next_in_queue(&mut self) -> Result<()> {
//         const PREVENT_SKIP_BEYOND_QUEUE_LENGTH: usize = 1;
//         if self.playback_queue.lock().len() == 0 {
//             return Ok(());
//         }
//         if self.playback_index < self.playback_queue.lock().len() - PREVENT_SKIP_BEYOND_QUEUE_LENGTH
//         {
//             self.playback_index += 1;
//         }
//         self.play_song_from_queue()?;
//         Ok(())
//     }

//     fn prev_in_queue(&mut self) -> Result<()> {
//         if self.playback_index > 0 {
//             self.playback_index -= 1;
//         }
//         self.play_song_from_queue()?;
//         Ok(())
//     }

//     // async fn load_library(&mut self) {
//     //     let load_path = self.global_settings.library_file.clone();
//     //     let library = Arc::clone(&self.music_library);
//     //     Library::read_from_file(&load_path)
//     //         .map(|new_lib| {
//     //             let mut lib = library.lock();
//     //             *lib = new_lib;
//     //         })
//     //         .map_err(|e| e.to_string());
//     //     // cx.notify();
//     // }

//     fn scan_and_save(&mut self) -> Result<(), String> {
//         //scan
//         self.music_library = Arc::new(Mutex::new(Library::new()));
//         let _ = self
//             .music_library
//             .lock()
//             .import_dir(&self.global_settings.folder_to_scan)
//             .map_err(|e| return e.to_string());
//         //save
//         let save_path = &self.global_settings.library_file;
//         self.music_library
//             .lock()
//             .save_to_file(&save_path)
//             .map_err(|e| return format!("SaveLibrary Error: {}", e))
//     }

//     fn read_or_create_config() -> GlobalSettings {
//         let settings = fs::read_to_string("Settings.toml");
//         match settings {
//             Ok(settings) => toml::from_str(&settings).unwrap_or(GlobalSettings::default()),
//             Err(err) => {
//                 println!("No Settings File: {}", err);
//                 GlobalSettings::default()
//             }
//         }
//     }
// }

// impl Render for Jukebox {
//     fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
//         cx.set_window_title("jukebox");
//         // cx.
//         // cx.on_action(cx.listener(Self::load_library));w
//         // cx.foreground_executor()
//         // .spawn(Library::load(&self.global_settings.library_file));

//         match self.clone().ui_state {
//             UIState::Loading => {
//                 ui::root().child(cx.new_view(|cx| LoadingUI {
//                     focus_handle: cx.focus_handle(),
//                     // jukebox: self.clone(),
//                 }))
//                 // .on_action(cx.listener(Self::load_library))
//             }
//             UIState::Main => ui::root().child(cx.new_view(|cx| MainUI {
//                 focus_handle: cx.focus_handle(),
//                 jukebox: self.clone(),
//             })),
//             UIState::Settings => ui::root().child(cx.new_view(|cx| SettingsUI {
//                 focus_handle: cx.focus_handle(),
//                 jukebox: self.clone(),
//             })),
//         }
//     }
// }

// fn main() {
//     App::new().run(|cx: &mut AppContext| {
//         let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);

//         cx.open_window(
//             WindowOptions {
//                 window_bounds: Some(WindowBounds::Windowed(bounds)),
//                 ..Default::default()
//             },
//             |cx| {
//                 cx.new_view(|_cx| Jukebox {
//                     ..Jukebox::default()
//                 })
//             },
//         )
//         .unwrap();
//     });
// }

// yoinked
actions!(main_menu, [Quit]);

fn init(app_state: Arc<AppState>, cx: &mut AppContext) -> Result<()> {
    app_workspace::init(app_state.clone(), cx);

    cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

    Ok(())
}

fn main() {
    let app_state = Arc::new(AppState {});

    let app = App::new().with_assets(Assets);

    app.run(move |cx| {
        AppState::set_global(Arc::downgrade(&app_state), cx);

        if let Err(e) = init(app_state.clone(), cx) {
            println!("{}", e);
            return;
        }

        cx.on_action(quit);

        cx.set_menus(vec![
            Menu {
                name: "GPUI App".into(),
                items: vec![MenuItem::action("Quit", Quit)],
            },
            Menu {
                name: "Edit".into(),
                items: vec![
                    MenuItem::os_action("Undo", Undo, gpui::OsAction::Undo),
                    MenuItem::os_action("Redo", Redo, gpui::OsAction::Redo),
                    MenuItem::separator(),
                    MenuItem::os_action("Cut", Cut, gpui::OsAction::Cut),
                    MenuItem::os_action("Copy", Copy, gpui::OsAction::Copy),
                    MenuItem::os_action("Paste", Paste, gpui::OsAction::Paste),
                ],
            },
        ]);
        cx.activate(true);

        app_workspace::open_new(app_state.clone(), cx, |_workspace, _cx| {
            // do something
        })
        .detach();
    });
}

fn quit(_: &Quit, cx: &mut AppContext) {
    cx.quit();
}

// // UI/Iced
// impl Application for Jukebox {
//     type Executor = executor::Default;
//     type Flags = ();
//     type Message = Message;
//     type Theme = Theme;

//     fn new(_flags: ()) -> (Self, Command<Message>) {
//         let app = Self::default();
//         (
//             app.clone(),
//             Command::perform(async move { app.load_library() }, Message::LoadComplete),
//         )
//     }

//     fn theme(&self) -> Theme {
//         Theme::Dark
//     }

//     fn title(&self) -> String {
//         String::from("Jukebox")
//     }

//     fn subscription(&self) -> Subscription<Self::Message> {
//         // TODO get key input (handle media keys)
//         const TICK_DURATION: f32 = 0.01;

//         let time =
//             iced::time::every(Duration::from_secs_f32(TICK_DURATION)).map(|_| Message::TickUpdate);

//         Subscription::batch([time])
//     }

//     fn update(&mut self, event: Message) -> Command<Message> {
//         match self.ui_state {
//             UIState::Loading => match event {
//                 Message::LoadComplete(result) => {
//                     match result {
//                         Ok(()) => {
//                             println!("Library loaded successfully.");
//                             self.ui_state = UIState::Main
//                         }
//                         Err(e) => {
//                             println!("Load failed: {}", e);
//                             self.ui_state = UIState::Main
//                         }
//                     }
//                     Command::none()
//                 }
//                 _ => Command::none(),
//             },
//             UIState::Main => match event {
//                 Message::TickUpdate => {
//                     self.update_time();
//                     Command::none()
//                 }
//                 Message::TogglePlayback => {
//                     if self.sink.lock().is_none() {
//                         let _ = self.play_song_from_queue();
//                     } else {
//                         self.toggle_sink_playback();
//                     }
//                     Command::none()
//                 }
//                 Message::AddTestSongToQueue => {
//                     self.add_song_to_queue_end(Song::new(PathBuf::from_str("./test.ogg").unwrap()))
//                         .expect("adding song to queue failed");
//                     Command::none()
//                 }
//                 Message::Scan => {
//                     let mut jb = self.clone();
//                     println!("scanning...");
//                     Command::perform(async move { jb.scan_and_save() }, Message::ScanComplete)
//                 }
//                 Message::ScanComplete(result) => {
//                     match result {
//                         Ok(()) => self.load_library().unwrap(),
//                         Err(e) => {
//                             format!("Scan failed: {}", e);
//                         }
//                     }
//                     Command::none()
//                 }
//                 Message::PickSong(id) => {
//                     self.add_song_to_queue_end(
//                         self.music_library.lock().songs.get(&id).unwrap().clone(),
//                     )
//                     .expect("adding song to queue failed");
//                     Command::none()
//                 }
//                 Message::LoadComplete(result) => {
//                     match result {
//                         Ok(()) => {
//                             println!("Library loaded successfully.");
//                         }
//                         Err(e) => {
//                             println!("Load failed: {}", e);
//                         }
//                     }
//                     Command::none()
//                 }
//                 Message::PreviousSong => {
//                     let _ = self.prev_in_queue();
//                     Command::none()
//                 }
//                 Message::NextSong => {
//                     let _ = self.next_in_queue();
//                     Command::none()
//                 }
//                 Message::ChangeUI(ui_state) => {
//                     self.ui_state = ui_state;
//                     Command::none()
//                 }
//                 _ => Command::none(),
//             },
//             UIState::Settings => match event {
//                 Message::SaveSettings(new_settings) => {
//                     self.global_settings = new_settings;
//                     Command::none()
//                 }
//                 Message::ChangeUI(ui_state) => {
//                     self.ui_state = ui_state;
//                     Command::none()
//                 }
//                 _ => Command::none(),
//             },
//         }
//     }

//     fn view(&self) -> Element<Message> {
//         match self.ui_state {
//             UIState::Loading => loading_ui(),
//             UIState::Main => main_ui(self.clone()),
//             UIState::Settings => settings_ui(self.global_settings.clone()),
//         }
//     }
// }
