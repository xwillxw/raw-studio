slint::include_modules!();

impl TrackData {
    fn new(new_name: &str, new_index: &str) -> TrackData {
        return TrackData {
            name: new_name.into(),
            index: new_index.into(),
            mute: false,
            solo: false
        }
    }
}

impl Project {
    
}

fn main() {
    let main_window = MainWindow::new().unwrap();
    let new_project = Project {
        name: "Test Project".into(),
        tempo: 120,
        tracks: [
            TrackData::new("Guitar", "1"),
            TrackData::new("Bass", "2"),
            TrackData::new("Drums", "3"),
        ].into()
    };
    main_window.set_project(new_project);
    main_window.run().unwrap();
}
