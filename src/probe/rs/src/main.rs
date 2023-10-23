// NOTE: need cocoa for some reason for objc to work
use cocoa;
use objc::{
    msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_foundation::{INSString, NSString};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::OpenOptions,
    io::{Read, Write},
};

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
struct App {
    name: String,
    pid: usize,
    hidden: bool,
}

fn main() {
    // Read state file
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("data.json")
        .unwrap();

    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    let mut target_app_ids: HashSet<usize> = if file_content.is_empty() {
        HashSet::new()
    } else {
        serde_json::from_str(&file_content).unwrap()
    };

    let mut apps = vec![];

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
            let app: *mut Object = msg_send![running_apps, objectAtIndex:i];

            // Convert name to String
            // https://github.com/SSheldon/rust-objc-foundation/blob/0.1.1/src/string.rs#L40-L50
            let name: *mut NSString = msg_send![app, localizedName];
            let bytes: *const std::os::raw::c_char = msg_send![name, UTF8String];
            let bytes = bytes as *const u8;
            let bytes = std::slice::from_raw_parts(bytes, NSString::len(&*name));
            let name = std::str::from_utf8(bytes).unwrap().to_owned();

            // Construct App and push to vector
            let pid: usize = msg_send![app, processIdentifier];
            // https://developer.apple.com/documentation/appkit/nsrunningapplication/1525949-ishidden
            let hidden: bool = msg_send![app, isHidden];
            apps.push(App {
                name: name.clone(),
                pid,
                hidden,
            });

            if !hidden {
                println!("Hiding active application {name}");
                let _: () = msg_send![app, hide]; // TODO: check if successful
                                                  // todo!();
                                                  // target_app_ids
                target_app_ids.insert(pid);
            } else {
                println!("Application {name} already hidden");
                if target_app_ids.contains(&pid) {
                    println!("Target {name} in list");
                    let _: () = msg_send![app, unhide]; // TODO: check if successful
                    target_app_ids.remove(&pid);
                }
            }
        }

        // Update state file
        let new_state = serde_json::to_string(&target_app_ids).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("data.json")
            .unwrap();
        file.write_all(new_state.as_bytes()).unwrap();

        let _: *mut Object = msg_send![autorelease_pool, release];
    }
}
