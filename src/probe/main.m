#import <Cocoa/Cocoa.h>

int main(int argc, const char * argv[]) {
    NSWorkspace *workspace = [NSWorkspace sharedWorkspace];
    NSString *configFilePath = [NSHomeDirectory() stringByAppendingPathComponent:@"hidden_apps.json"];

    NSError *error;
    NSMutableSet *hiddenAppIdentifiers = [NSMutableSet set];

    // Read the configuration file to get the list of hidden application process IDs
    NSData *data = [NSData dataWithContentsOfFile:configFilePath options:0 error:&error];
    if (data) {
        NSArray *hiddenProcessIDs = [NSJSONSerialization JSONObjectWithData:data options:0 error:nil];
        [hiddenAppIdentifiers addObjectsFromArray:hiddenProcessIDs];
    }

    // Toggle the visibility of applications
    for (NSRunningApplication *app in workspace.runningApplications) {
        if ([hiddenAppIdentifiers containsObject:@(app.processIdentifier)]) {
            [app unhide];
            [hiddenAppIdentifiers removeObject:@(app.processIdentifier)];
        } else {
            [app hide];
            [hiddenAppIdentifiers addObject:@(app.processIdentifier)];
        }
    }

    // Write the updated configuration file with the current hidden application process IDs
    NSArray *updatedHiddenProcessIDs = [hiddenAppIdentifiers.allObjects valueForKeyPath:@"@unionOfObjects.integerValue"];
    NSData *updatedData = [NSJSONSerialization dataWithJSONObject:updatedHiddenProcessIDs options:0 error:nil];
    [updatedData writeToFile:configFilePath atomically:YES];

    return 0;
}
