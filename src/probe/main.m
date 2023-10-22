#import <Cocoa/Cocoa.h>

// TODO: bring them back

int main(int argc, const char * argv[]) {
    NSWorkspace *workspace = [NSWorkspace sharedWorkspace];
    for (NSRunningApplication *app in workspace.runningApplications) {
        [app hide];
    }
    return 0;
}

