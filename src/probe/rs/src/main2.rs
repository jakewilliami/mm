use clap::{crate_authors, crate_version, Parser};
#[allow(unused_imports)]
use cocoa; // NOTE: need Cocoa for NSWorkspace
use core_foundation::base::*;
use core_foundation::number::*;
use core_foundation::string::*;
use core_graphics::display::*;
use indexmap::IndexSet;
use objc::{
    msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_foundation::{INSString, NSString};
use serde::{Deserialize, Serialize};
use std::ffi::{c_void, CStr};
use std::{
    collections::{HashMap, HashSet},
    fs::OpenOptions,
    io::{Read, Write},
};

// TODO: option to reset state file
// TODO: make less buggy/unreliable (better persist?)
// TODO: add flag to specify toggle mode rather than infer it
// TODO: add dry-run flag to log but not hide (good for development)
// TODO: add logging

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
    windows: IndexSet<usize>,
}

impl AppState {
    fn new() -> Self {
        Self {
            mode: ToggleMode::Hide,
            pids: HashMap::new(),
            windows: IndexSet::new(),
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

#[derive(Eq, PartialEq, Hash, Debug)]
struct ActiveWindow {
    name: Option<String>,
    pid: usize,
}

fn main() {
    let _cli = Cli::parse();

    // let apps = get_ru nning_apps();
    // let active_windows = get_active_windows();
    // for (pid, window) in active_windows.iter() {
    //     for (pid2, app) in &apps {
    //         if pid == pid2 {
    //             println!("{:?}: {:?}, {:?}", pid, window.name, app.name);
    //         }
    //     }
    //     // if let Some(name) = window.name {
    //     // println!("{name}: {pid}");
    //     // }
    // }
    // return;

    // let windows = get_active_windows();
    // for (_, app) in get_all_running_apps() {
    //     if !app.is_hidden {
    //         for window in windows.iter() {
    //             if window.pid == app.pid {
    //                 println!("{} ({})", app.name, app.pid);
    //             }
    //         }
    //     }
    // }
    // return;

    // let mut apps = HashMap::new();
    // let windows = get_active_windows();
    // let running_apps = get_all_running_apps();

    // for window in windows {
    //     if let Some(app) = running_apps.get(&window.pid) {
    //         apps.insert(window.pid, app);
    //     }
    // }

    // for (_, app) in apps {
    // println!("{app:?}");
    // }
    // return;

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

    let mut running_apps = HashMap::new();
    let active_windows = get_active_windows();
    let all_running_apps = get_all_running_apps();

    for window in active_windows.iter() {
        if let Some(app) = all_running_apps.get(&window.pid) {
            running_apps.insert(window.pid, app);
        }
    }

    // for (_, app) in running_apps {
    // println!(
    // "[isHidden: {}] {} ({})",
    // if app.is_hidden { "true " } else { "false" },
    // app.name,
    // app.pid
    // );
    // }
    return;

    // If we are in "hiding" mode, first we need to take note of the already
    // hidden applications, as well as the window order
    if app_state.mode == ToggleMode::Hide {
        // Record initial state of running apps
        for (pid, app) in running_apps.iter() {
            if app.is_hidden {
                app_state
                    .pids
                    .entry(HiddenState::AlreadyHidden)
                    .or_default()
                    .insert(*pid);
            }
        }

        // Record window order
        // NOTE: hidden apps will not be recorded here
        for window in active_windows.iter() {
            app_state.windows.insert(window.pid);
        }
    }

    running_apps.drain();
    for window in active_windows.iter() {
        if let Some(app) = all_running_apps.get(&window.pid) {
            running_apps.insert(window.pid, app);
        }
    }
    println!("new: {:?}", running_apps);

    // Now process remaining applications
    // We can hide or show according to the mode
    match app_state.mode {
        // If we are in hiding mode, we check if the app is hidden or not.  If the app
        // is not hidden, we need to hide it, and make sure we add it to the list of
        // hidden apps
        ToggleMode::Hide => {
            for (pid, app) in running_apps.iter() {
                if !app.is_hidden {
                    // if app.is
                    //&& app_state.windows.contains(pid) {
                    // TODO: fix bug where apps don't close sometimes, no idea why or
                    //   how to reproduce
                    println!("Hiding application {} ({})", app.name, pid);
                    let _: () = unsafe { msg_send![app.instance, hide] };
                    app_state
                        .pids
                        .entry(HiddenState::Hidden)
                        .or_default()
                        .insert(*pid);
                }
            }
        }

        // If we are in show mode, we need to show the application and remove
        // it from the hidden state
        ToggleMode::Show => {
            // Hide apps that have associated windows
            // If the app has an active window then we handle it separately
            // Order doesn't matter here
            for (pid, app) in running_apps.iter() {
                if app.is_hidden && !app_state.windows.contains(pid) {
                    println!("Unhiding windowless application {} ({})", app.name, app.pid);
                    let _: () = unsafe { msg_send![app.instance, unhide] };
                    app_state.pids.entry(HiddenState::Hidden).and_modify(|set| {
                        set.remove(&app.pid);
                    });
                }
            }

            for window_pid in app_state.windows.iter().rev() {
                // If the app was already hidden, we can ignore it
                if let Some(set) = app_state.pids.get(&HiddenState::AlreadyHidden) {
                    if set.contains(window_pid) {
                        continue;
                    }
                }

                if let Some(app) = running_apps.get(window_pid) {
                    println!("Unhiding windowed application {} ({})", app.name, app.pid);
                    if app.is_hidden {
                        let _: () = unsafe { msg_send![app.instance, unhide] };
                        app_state.pids.entry(HiddenState::Hidden).and_modify(|set| {
                            set.remove(&app.pid);
                        });
                        // app_state.windows.shift_remove(&app.pid);
                    }
                }
            }
        }
    }

    // Update state file, or empty it if we are finished one toggle cycle
    let new_state = serde_json::to_string(&app_state).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("data.json")
        .unwrap();

    if app_state.mode == ToggleMode::Hide {
        file.write_all(new_state.as_bytes()).unwrap();
    }
}

fn get_all_running_apps() -> HashMap<usize, RunningApp> {
    let mut apps = HashMap::new();

    let autorelease_pool_cls = Class::get("NSAutoreleasePool").unwrap();
    let autorelease_pool: *mut Object = unsafe { msg_send![autorelease_pool_cls, new] };
    let workspace_cls = Class::get("NSWorkspace").unwrap();
    let workspace: *mut Object = unsafe { msg_send![workspace_cls, sharedWorkspace] };

    // Collect information about running apps:
    //   https://developer.apple.com/documentation/appkit/nsrunningapplication
    //   https://github.com/mrmekon/fruitbasket/blob/master/src/osx.rs
    let running_apps: *mut Object = unsafe { msg_send![workspace, runningApplications] };
    let n_apps = unsafe { msg_send![running_apps, count] };

    for i in 0..n_apps {
        let app_instance: *mut Object = unsafe { msg_send![running_apps, objectAtIndex:i] };
        let name: String = get_app_name(app_instance);
        let pid: usize = unsafe { msg_send![app_instance, processIdentifier] };
        let is_hidden: bool = unsafe { msg_send![app_instance, isHidden] };
        // let is_hidden: bool = unsafe { app_instance.get_ivar("isHidden") };

        println!(
            "[isHidden: {}] {} ({})",
            if is_hidden { "true " } else { "false" },
            name,
            pid
        );

        apps.insert(
            pid,
            RunningApp {
                instance: app_instance,
                name,
                pid,
                is_hidden,
            },
        );
    }

    let _: *mut Object = unsafe { msg_send![autorelease_pool, release] };

    apps
}

// Get app name from app instance
//
// Ref on converting string:
//   https://github.com/SSheldon/rust-objc-foundation/blob/0.1.1/src/string.rs#L40-L50
//
// TODO: research using std::ffi::Cstr intead
fn get_app_name(app: *mut Object) -> String {
    let name: *mut NSString = unsafe { msg_send![app, localizedName] };
    let bytes: *const std::os::raw::c_char = unsafe { msg_send![name, UTF8String] };
    let bytes = bytes as *const u8;
    let bytes = unsafe { std::slice::from_raw_parts(bytes, NSString::len(&*name)) };
    std::str::from_utf8(bytes).unwrap().to_owned()
}

// Get active windows in order of last-used:
//   https://stackoverflow.com/a/46947382
//
// See alternative references for obtaining windows in order of use:
//   https://stackoverflow.com/a/3001507
//   https://stackoverflow.com/a/1226889
//   https://gist.github.com/0xced/163918
//
// Importantly, this returns an IndexMap so that insertion order is remembered:
//   https://stackoverflow.com/a/66858810
// Which is to say, the least recently used apps will appear in the set first
fn get_active_windows() -> IndexSet<ActiveWindow> {
    const OPTIONS: CGWindowListOption =
        kCGWindowListOptionOnScreenOnly | kCGWindowListExcludeDesktopElements;
    let window_list = unsafe { CGWindowListCopyWindowInfo(OPTIONS, kCGNullWindowID) };
    let n_windows = unsafe { CFArrayGetCount(window_list) };

    let mut windows = IndexSet::new();

    for i in (0..n_windows).rev() {
        let window = unsafe { CFArrayGetValueAtIndex(window_list, i) as CFDictionaryRef };
        let name = get_window_name(window);
        let pid = get_window_pid(window);

        // If window process ID is None, then wtf am I supposed to do about it?
        // Stop coming at me.  Came here to have a good time.  Ignore it!
        if let Some(pid) = pid {
            windows.insert(ActiveWindow { name, pid });
        }
    }

    unsafe { CFRelease(window_list as CFTypeRef) }

    windows
}

// Something TODO DOCUMENTATION:
//   https://stackoverflow.com/a/60140186
fn get_window_name(dict_ref: CFDictionaryRef) -> Option<String> {
    let key = CFString::new("kCGWindowOwnerName");
    let mut value: *const c_void = std::ptr::null();

    if unsafe { CFDictionaryGetValueIfPresent(dict_ref, key.to_void(), &mut value) != 0 } {
        let cf_ref = value as CFStringRef;
        let c_ptr = unsafe { CFStringGetCStringPtr(cf_ref, kCFStringEncodingUTF8) };
        if !c_ptr.is_null() {
            let c_result = unsafe { CStr::from_ptr(c_ptr) };
            return Some(String::from(c_result.to_str().unwrap()));
        }
    }

    None
}

fn get_window_pid(dict_ref: CFDictionaryRef) -> Option<usize> {
    let key = CFString::new("kCGWindowOwnerPID");
    let mut value: *const c_void = std::ptr::null();

    if unsafe { CFDictionaryGetValueIfPresent(dict_ref, key.to_void(), &mut value) != 0 } {
        let cf_ref = value as CFNumberRef;
        let mut number: usize = 0;
        let c_ptr = unsafe {
            CFNumberGetValue(
                cf_ref,
                kCFNumberSInt32Type,
                &mut number as *mut usize as *mut c_void,
            )
        };
        if c_ptr {
            return Some(number);
        }
    }

    None
}
