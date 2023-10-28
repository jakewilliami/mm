use clap::{crate_authors, crate_version, Parser};
use cocoa; // NOTE: need cocoa for some reason for objc to work
use objc::{
    msg_send,
    runtime::{Class, Object, BOOL},
    sel, sel_impl,
};
use objc_foundation::{INSString, NSString};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs::OpenOptions,
    io::{Read, Write},
};

// TODO: option to reset state file
// TODO: ensure windows appear in same order when brought back
// TODO: make less buggy/unreliable (better persist?)
// TODO: add flag to specify toggle mode rather than infer it

#[derive(Parser)]
#[command(name = "mm", author = crate_authors!(", "), version = crate_version!())]
/// Mischeif Managed: toggle desktop view in macOS
struct Cli {}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
enum ToggleMode {
    Hide,
    Show,
}

impl ToggleMode {
    fn toggle(&self) -> Self {
        match self {
            Self::Hide => Self::Show,
            Self::Show => Self::Hide,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
enum HiddenState {
    Hidden,
    NotHidden, // TODO: may not need this state
    AlreadyHidden,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppState {
    mode: ToggleMode,
    pids: HashMap<HiddenState, HashSet<usize>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            mode: ToggleMode::Hide,
            pids: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct RunningApp {
    instance: *mut Object,
    name: String,
    pid: usize,
    is_hidden: bool,
}

fn main() {
    let _cli = Cli::parse();

    // Read state file
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("data.json")
        .unwrap();

    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    // TODO: check if schema has changed e.g. AppState might be different to what's on the fs
    let mut app_state: AppState = if file_content.is_empty() {
        AppState::new()
    } else {
        let mut app_state: AppState = serde_json::from_str(&file_content).unwrap();

        // If we are reading the app state from the fs, then we need to toggle the mode to
        // the opposite of last time it was run (i.e., when the state was persisted to disk)
        app_state.mode = app_state.mode.toggle();
        app_state
    };

    // If we are in "hiding" mode, first we need to take note of the already
    // hidden applications
    if app_state.mode == ToggleMode::Hide {
        for app in get_running_apps() {
            if app.is_hidden {
                app_state
                    .pids
                    .entry(HiddenState::AlreadyHidden)
                    .or_insert(HashSet::new())
                    .insert(app.pid);
            }
        }
    }

    // Now we can hide or show according to the mode
    for app in get_running_apps() {
        // If the app was already hidden, we can ignore it
        if let Some(set) = app_state.pids.get(&HiddenState::AlreadyHidden) {
            if set.contains(&app.pid) {
                continue;
            }
        }

        // Now process remaining applications
        match app_state.mode {
            ToggleMode::Hide => {
                // If we are in hiding mode, we check if the app is hidden or not.
                // If the app is not hidden, we need to hide it, and make sure we
                // add it to the list of hidden apps
                if !app.is_hidden {
                    // TODO: fix bug where apps don't close sometimes
                    // let app_instance = app.instance;
                    let _res: BOOL = unsafe { msg_send![app.instance, hide] };
                    app_state
                        .pids
                        .entry(HiddenState::Hidden)
                        .or_insert(HashSet::new())
                        .insert(app.pid);
                }
            }
            ToggleMode::Show => {
                // If we are in show mode, we need to show the application and remove
                // it from the hidden state
                if app.is_hidden {
                    let _: () = unsafe { msg_send![app.instance, unhide] };
                    app_state.pids.entry(HiddenState::Hidden).and_modify(|set| {
                        set.remove(&app.pid);
                    });
                }
            }
        }
    }

    // TODO: empty the AlreadyHidden HashSet if we are at the end of Show mode

    // Update state file
    let new_state = serde_json::to_string(&app_state).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("data.json")
        .unwrap();
    file.write_all(new_state.as_bytes()).unwrap();
}

fn get_running_apps() -> Vec<RunningApp> {
    let mut apps = Vec::new();

    unsafe {
        let autorelease_pool_cls = Class::get("NSAutoreleasePool").unwrap();
        let autorelease_pool: *mut Object = msg_send![autorelease_pool_cls, new];
        let workspace_cls = Class::get("NSWorkspace").unwrap();
        let workspace: *mut Object = msg_send![workspace_cls, sharedWorkspace];

        // Collect information about running apps
        // https://developer.apple.com/documentation/appkit/nsrunningapplication
        // https://github.com/mrmekon/fruitbasket/blob/master/src/osx.rs
        let running_apps: *mut Object = msg_send![workspace, runningApplications];
        let n_apps = msg_send![running_apps, count];

        for i in 0..n_apps {
            let app_instance: *mut Object = msg_send![running_apps, objectAtIndex:i];
            let name: String = get_app_name(app_instance);
            let pid: usize = msg_send![app_instance, processIdentifier];
            let is_hidden: bool = msg_send![app_instance, isHidden];

            apps.push(RunningApp {
                instance: app_instance,
                name,
                pid,
                is_hidden,
            });
        }

        let _: *mut Object = msg_send![autorelease_pool, release];
    }

    return apps;
}

fn get_app_name(app: *mut Object) -> String {
    unsafe {
        let name: *mut NSString = msg_send![app, localizedName];
        let bytes: *const std::os::raw::c_char = msg_send![name, UTF8String];
        let bytes = bytes as *const u8;
        let bytes = std::slice::from_raw_parts(bytes, NSString::len(&*name));
        std::str::from_utf8(bytes).unwrap().to_owned()
    }
}
