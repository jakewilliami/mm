#import <Cocoa/Cocoa.h>
#import <Carbon/Carbon.h>
#import <dlfcn.h>
#include <CoreFoundation/CoreFoundation.h>
#include <ApplicationServices/ApplicationServices.h>

// Print currently visible applications
int main(int argc, const char *argv[]) {
    @autoreleasepool {
        NSWorkspace *workspace = [NSWorkspace sharedWorkspace];
        NSArray *apps = [workspace runningApplications];

        NSArray *options = @[@(kCGWindowListOptionOnScreenOnly), @(kCGWindowListExcludeDesktopElements)];
        CFArrayRef windowList = CGWindowListCopyWindowInfo(kCGWindowListOptionAll, kCGNullWindowID);
        NSArray *windows = (__bridge NSArray *)windowList;
        NSMutableSet *processedPIDs = [NSMutableSet set];

        for (NSDictionary *window in windows) {
            NSString *name = window[(__bridge NSString *)kCGWindowOwnerName];
            pid_t windowPID = [window[(__bridge NSString *)kCGWindowOwnerPID] intValue];

            // Check if we have already processed this PID
            if (![processedPIDs containsObject:@(windowPID)]) {
                for (NSRunningApplication *app in apps) {
                    if (app.processIdentifier == windowPID) {
                        NSLog(@"[isHidden: %@] %@ (%d)", app.isHidden ? @"true " : @"false", app.localizedName, app.processIdentifier);
                        break;
                    }
                }

                // Add the PID to the processed set
                [processedPIDs addObject:@(windowPID)];
            }
        }
    }

    return 0;
}
