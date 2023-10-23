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

            let pid: usize = msg_send![app, processIdentifier];
            let is_hidden: bool = msg_send![app, isHidden];

            // If the application is active (the first branch of the if statement), then we want to hide it
            // and add its ID to the list.  This is the first stage of the toggle.  The second stage of the
            // toggle is a little more complicated (this is the second branch of the if statement).  We need
            // to check if the hidden application is in the list.  If it is, then we have hidden it with the
            // present application, and we need to unhide it accordingly.  If it's not in the list, then it
            // was already hidden, so we can safely ignore it.
            if !is_hidden {
                let _: () = msg_send![app, hide];
                target_app_ids.insert(pid);
            } else {
                if target_app_ids.contains(&pid) {
                    let _: () = msg_send![app, unhide];
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
