// TODO: print hidden file option
// TODO: refresh hidden file option
// TODO: persist to tmp

#import <Cocoa/Cocoa.h>

int main(int argc, const char * argv[]) {
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
