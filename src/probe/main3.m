// TODO: print hidden file option
// TODO: refresh hidden file option
// TODO: persist to tmp

#import <Cocoa/Cocoa.h>
int main(int argc, const char * argv[]) {
    @autoreleasepool {
        NSWorkspace *workspace = [NSWorkspace sharedWorkspace];
        NSArray *runningApps = [workspace runningApplications];
        
        for (NSRunningApplication *app in runningApps) {
            NSLog(@"Application Name: %@", [app localizedName]);
            NSLog(@"Application ID: %@", [app bundleIdentifier]);
        }
    }
    
    return 0;
}

int main1111(int argc, const char * argv[]) {
    @autoreleasepool {
        // Get a list of running applications
        NSArray *runningApps = [[NSWorkspace sharedWorkspace] runningApplications];
        
        for (NSRunningApplication *app in runningApps) {
            // Get the process identifier (PID) of the application
            pid_t pid = [app processIdentifier];
            
            // Create an accessibility object for the application using its PID
            AXUIElementRef appElement = AXUIElementCreateApplication(pid);
            
            // Get the list of windows for the application
            CFArrayRef windows;
            AXUIElementCopyAttributeValue(appElement, kAXWindowsAttribute, (CFTypeRef *)&windows);
            
            for (NSInteger i = 0; i < CFArrayGetCount(windows); i++) {
                // Get a reference to the window
                AXUIElementRef windowElement = CFArrayGetValueAtIndex(windows, i);
                
                // Get the window title
                CFStringRef title;
                AXUIElementCopyAttributeValue(windowElement, kAXTitleAttribute, (CFTypeRef *)&title);
                NSString *windowTitle = (__bridge NSString *)title;
                
                // Check if the window is currently focused (on top)
                Boolean isFocused;
                AXUIElementIsAttributeSettable(windowElement, kAXFocusedAttribute, &isFocused);
                
                if (isFocused) {
                    NSLog(@"Window Title (On Top): %@", windowTitle);
                } else {
                    NSLog(@"Window Title: %@", windowTitle);
                }
            }
            
            // Release the application element
            CFRelease(appElement);
        }
    }
    return 0;
}


int main111(int argc, const char * argv[]) {
    @autoreleasepool {
        NSWorkspace *workspace = [NSWorkspace sharedWorkspace];
        NSArray *runningApps = [workspace runningApplications];
        
        for (NSRunningApplication *app in runningApps) {
            NSLog(@"Application Name: %@", [app localizedName]);
            NSLog(@"Application ID: %@", [app bundleIdentifier]);
            
            pid_t processIdentifier = [app processIdentifier];
            AXUIElementRef appElement;
            AXUIElementCopyElementAtPosition(kOnSystemDisk, 0, 0, &appElement);
            
            if (appElement) {
                CFArrayRef windowList;
                AXUIElementCopyAttributeValues(appElement, kAXWindowsAttribute, 0, 100, &windowList);
                
                for (CFIndex i = 0; i < CFArrayGetCount(windowList); i++) {
                    AXUIElementRef windowElement = CFArrayGetValueAtIndex(windowList, i);
                    
                    CFStringRef windowTitle = NULL;
                    AXUIElementCopyAttributeValue(windowElement, kAXTitleAttribute, (CFTypeRef *)&windowTitle);
                    
                    if (windowTitle) {
                        NSLog(@"Window Title: %@", (__bridge NSString *)windowTitle);
                        CFRelease(windowTitle);
                    }
                }
                
                CFRelease(windowList);
                CFRelease(appElement);
            }
        }
    }
    
    return 0;
}

int main11(int argc, const char * argv[]) {
    @autoreleasepool {
        NSWorkspace *workspace = [NSWorkspace sharedWorkspace];
        NSArray *runningApps = [workspace runningApplications];
        
        for (NSRunningApplication *app in runningApps) {
            NSLog(@"Application Name: %@", [app localizedName]);
            NSLog(@"Application ID: %@", [app bundleIdentifier]);
            
            AXUIElementRef appElement = AXUIElementCreateApplication([app processIdentifier]);
            if (appElement) {
                CFArrayRef windows;
                AXUIElementCopyAttributeValues(appElement, kAXWindowsAttribute, 0, 100, &windows);
                
                for (NSInteger i = 0; i < CFArrayGetCount(windows); i++) {
                    AXUIElementRef windowElement = CFArrayGetValueAtIndex(windows, i);
                    CFStringRef windowTitle;
                    if (AXUIElementCopyAttributeValue(windowElement, kAXTitleAttribute, (CFTypeRef *)&windowTitle) == kAXErrorSuccess) {
                        NSLog(@"Window Title: %@", (__bridge NSString *)windowTitle);
                        CFRelease(windowTitle);
                    }
                }
                
                CFRelease(windows);
                CFRelease(appElement);
            }
        }
    }
    
    return 0;
}

int main2(int argc, const char * argv[]) {
    NSString *configFilePath = [NSHomeDirectory() stringByAppendingPathComponent:@"hidden_apps.json"];
    NSError *error;

    // Read state file
    NSMutableSet *targetAppIDs = [NSMutableSet set];
    NSData *data = [NSData dataWithContentsOfFile:configFilePath options:0 error:&error];
    if (data) {
        NSArray *mmStateFS = [NSJSONSerialization JSONObjectWithData:data options:0 error:nil];
        [targetAppIDs addObjectsFromArray:mmStateFS];
    }

    // Toggle the visibility of applications
    for (NSRunningApplication *app in [NSWorkspace sharedWorkspace].runningApplications) {
        // If the application is active (the first branch of the if statement), then we want to hide it
        // and add its ID to the list.  This is the first stage of the toggle.  The second stage of the
        // toggle is a little more complicated (this is the second branch of the if statement).  We need
        // to check if the hidden application is in the list.  If it is, then we have hidden it with the
        // present application, and we need to unhide it accordingly.  If it's not in the list, then it
        // was already hidden, so we can safely ignore it.

        // NSLog(@"Processing application: %@ (%d)", app.localizedName, app.processIdentifier);
        if (![app isHidden]) {
            NSLog(@"Hiding active application: %@ (%d)", app.localizedName, app.processIdentifier);
            [app hide];
            [targetAppIDs addObject:@(app.processIdentifier)];
        } else {
            NSLog(@"Application already hidden: %@ (%d)", app.localizedName, app.processIdentifier);
            if ([targetAppIDs containsObject:@(app.processIdentifier)]) {
                NSLog(@"Target in list: %@ (%d)", app.localizedName, app.processIdentifier);
                [app unhide];
                [targetAppIDs removeObject:@(app.processIdentifier)];
            }
        }
    }

    // Update state file
    NSArray *newTargetAppIDs = [targetAppIDs.allObjects valueForKeyPath:@"@unionOfObjects.integerValue"];
    NSData *updatedData = [NSJSONSerialization dataWithJSONObject:newTargetAppIDs options:0 error:nil];
    [updatedData writeToFile:configFilePath atomically:YES];

    return 0;
}
