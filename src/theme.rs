use serde_json::json;

// TODO: all these themes need to go into a config file
// the activeProgramId is completely arbitrary, when you load a theme
// onto the pixelblaze, it generates an id
// so these definitions should live on the filesystem or something

// this file represents the translation of a theme (string) to an entire parameter set
// sent to the pixelblaze.  These can be made in the pixelblaze UI and then queried through
// the API.
pub fn bluegreen(brightness: f32) -> String {
    json!({
        "brightness": brightness,
        "activeProgramId": "HbjL4SMNcdpCWPN2E",
        "setVars": {
            "speed": 1.4,
            "waveLength": 0.90,
            "depth": 0.06
        },
        "setControls": {
            "hsvPickerHue": [ 0.61, 0.73, 0.95 ]
        }
    })
    .to_string()
}

pub fn orange(brightness: f32) -> String {
    json!({
        "brightness": brightness,
        "activeProgramId": "K58J7fPWpqXjeoLsW",
        "setVars": {
            "nCurrentLight":0
        }
    })
    .to_string()
}

pub fn blacklight(brightness: f32) -> String {
    json!({
        "brightness": brightness,
        "activeProgramId": "HbjL4SMNcdpCWPN2E",
        "setVars": {
            "speed": 1.28,
            "waveLength": 2,
            "depth": 0.1
        },
        "setControls": {"hsvPickerHue":[0.8, 0.95, 0.70]}
    })
    .to_string()
}

pub fn white(brightness: f32) -> String {
    json!({
        "brightness": brightness,
        "activeProgramId": "K58J7fPWpqXjeoLsW",
        "setVars": { "nCurrentLight": 3 }
    })
    .to_string()
}
