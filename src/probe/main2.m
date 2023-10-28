#import <Cocoa/Cocoa.h>

int main(int argc, const char * argv[]) {
    NSWorkspace *workspace = [NSWorkspace sharedWorkspace];
    for (NSRunningApplication *app in workspace.runningApplications) {
        NSLog(@"%d (%d) %@", [app isHidden], app.processIdentifier, app.localizedName);
    }
    return 0;
}

