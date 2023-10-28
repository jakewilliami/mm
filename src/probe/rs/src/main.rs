use cocoa;
use core_foundation::array::CFArrayRef;
use core_graphics::display::*;
use core_graphics::{
    display::{
        kCGNullWindowID, kCGWindowListExcludeDesktopElements, kCGWindowListOptionAll,
        kCGWindowListOptionOnScreenOnly, CFArrayGetCount, CFArrayGetValueAtIndex,
        CGWindowListCopyWindowInfo,
    },
    window::kCGWindowOwnerName,
};
use objc::{
    msg_send,
    runtime::{Class, Object, Sel, BOOL},
    sel, sel_impl,
};
use objc_foundation::{INSArray, INSString, NSArray, NSString};

use core_foundation::base::*;
use core_foundation::number::*;
use core_foundation::string::*;
use std::ffi::{c_void, CStr};

/*
int main (int argc, const char * argv[]) {
    @autoreleasepool {
        NSArray *options = @[@(kCGWindowListOptionOnScreenOnly), @(kCGWindowListExcludeDesktopElements)];
        CFArrayRef windowList = CGWindowListCopyWindowInfo(kCGWindowListOptionAll, kCGNullWindowID);
        NSArray *windows = (__bridge NSArray *)windowList;

        for (NSDictionary *window in windows) {
            NSString *name = window[(__bridge NSString *)kCGWindowOwnerName];
            if (name) {
                NSLog(@"%@", name);
            }
            pid_t windowPID = [window[(__bridge NSString *)kCGWindowOwnerPID] intValue];
            NSLog(@"%d", windowPID);
        }
    }

    return 0;
}
 */

// const kCGWindowListOptionOnScreenOnly: i32 = 1;
// const kCGWindowListExcludeDesktopElements: i32 = 2;

fn main() {
    // let kCGWindowListOptionOnScreenOnly: i32 = unsafe {
    // let class = class!("objc_runtime_apple_CGWindowListOption");
    // let selector = sel!(kCGWindowListOptionOnScreenOnly);
    // let option: BOOL = msg_send![class, performSelector:selector];
    // option as i32
    // };

    unsafe {
        // let options = NSArray::from_vec(vec![
        // Id < kCGWindowListOptionOnScreenOnly,
        // Id<kCGWindowListExcludeDesktopElements>,
        // ]);
        let options = vec![
            kCGWindowListOptionOnScreenOnly,
            kCGWindowListExcludeDesktopElements,
        ];

        let window_list = CGWindowListCopyWindowInfo(kCGWindowListOptionAll, kCGNullWindowID);
        // let window_list: *mut Object = msg_send![
        // CGWindowListCopyWindowInfo,
        // kCGWindowListOptionAll,
        // kCGNullWindowID
        // ];

        /*for (NSDictionary *window in windows) {
                NSString *name = window[(__bridge NSString *)kCGWindowOwnerName];
                if (name) {
                    NSLog(@"%@", name);
                }
                pid_t windowPID = [window[(__bridge NSString *)kCGWindowOwnerPID] intValue];
                NSLog(@"%d", windowPID);
        }*/

        let n_windows = CFArrayGetCount(window_list);

        for i in 0..n_windows {
            let window = CFArrayGetValueAtIndex(window_list, i) as CFDictionaryRef;
            let name = get_window_name(window);
            let pid = get_window_pid(window);
            println!("{:?}, {:?}", name, pid)

            // let name = kCGWindowOwnerName(window);
            // let name = CFArrayGetValueAtIndex(window, kCGWindowOwnerName);
            // println!("{:?}", kCGWindowOwnerName);
        }

        CFRelease(window_list as CFTypeRef);
        // println!("{n_windows} applications running");

        // let windows: *mut Object = msg_send![window_list, as_ref];
    }
}

// https://stackoverflow.com/a/60140186
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
    return None;
}

fn get_window_pid(dict_ref: CFDictionaryRef) -> Option<u64> {
    let key = CFString::new("kCGWindowOwnerPID");
    let mut value: *const c_void = std::ptr::null();

    if unsafe { CFDictionaryGetValueIfPresent(dict_ref, key.to_void(), &mut value) != 0 } {
        let cf_ref = value as CFNumberRef;
        let mut number: u64 = 0;
        let c_ptr = unsafe {
            CFNumberGetValue(
                cf_ref,
                kCFNumberSInt64Type,
                &mut number as *mut u64 as *mut c_void,
            )
        };
        if c_ptr {
            return Some(number);
        }
    }
    return None;
}

/*fn main2() {
    unsafe {
        let _pool = Class::get("NSAutoreleasePool").unwrap();
        let options: Vec<u32> = vec![1, 2]; // kCGWindowListOptionOnScreenOnly, kCGWindowListExcludeDesktopElements
        let window_list_cls = Class::get("CGWindowList");
        let window_list: *mut Object = msg_send![
            window_list_cls,
            copyWindowInfo,
            0,
            options.as_ptr(),
            options.len() as u64
        ];
        let windows: *mut Object = msg_send![window_list, copy];

        for i in 0..1 {
            let window: *mut Object = msg_send![windows, objectAtIndex: i as u64];
            let name: *mut Object =
                msg_send![window, objectForKey: Sel::register("kCGWindowOwnerName")];
            if !name.is_null() {
                let name_str: *const i8 = msg_send![name, UTF8String];
                let name_rust_str = std::ffi::CStr::from_ptr(name_str).to_string_lossy();
                println!("{}", name_rust_str);
            }
            let pid: *mut Object =
                msg_send![window, objectForKey: Sel::register("kCGWindowOwnerPID")];
            if !pid.is_null() {
                let pid_number: i32 = msg_send![pid, intValue];
                println!("{}", pid_number);
            }
        }
    }
}*/
