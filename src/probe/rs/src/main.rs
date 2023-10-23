// NOTE: need cocoa for some reason for objc to work
use cocoa;

use objc::{
    msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};

use objc_foundation::{INSString, NSString};
// use objc_id::Id;

#[derive(Debug)]
struct App {
    name: String,
    pid: usize,
    hidden: bool,
}

fn main() {
    let mut apps = vec![];

    unsafe {
        let autorelease_pool_cls = Class::get("NSAutoreleasePool").unwrap();
        let autorelease_pool: *mut Object = msg_send![autorelease_pool_cls, new];
        let workspace_cls = Class::get("NSWorkspace").unwrap();
        let workspace: *mut Object = msg_send![workspace_cls, sharedWorkspace];

        // Collect information about running apps
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
            let hidden: bool = msg_send![app, isHidden];
            apps.push(App { name, pid, hidden });
        }

        let _: *mut Object = msg_send![autorelease_pool, release];
    }

    println!("{:#?}", apps);
}
