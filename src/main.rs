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
    fn new(new_name: &str, new_tempo: i32, new_tracks: &[TrackData]) -> Project {
        return Project { 
            name: new_name.into(),
            tempo: new_tempo,
            tracks: new_tracks.into()
        }
    }
}

fn main() {
    let main_window = MainWindow::new().unwrap();
    let new_tracks = [
        TrackData::new("Guitar", "1"),
        TrackData::new("Bass", "2"),
        TrackData::new("Drums", "3"),
    ];
    let new_project = Project::new(
        "Test Project",
        160,
        &new_tracks
    );
    main_window.set_project(new_project);
    main_window.run().unwrap();
}
