#import <Carbon/Carbon.h>
#import <dlfcn.h>
// #import <Cocoa/Cocoa.h>
// #include <CoreFoundation/CoreFoundation.h>

/*
 * Returns an array of CFDictionaryRef types, each of which contains information about one of the processes.
 * The processes are ordered in front to back, i.e. in the same order they appear when typing command + tab, from left to right.
 * See the ProcessInformationCopyDictionary function documentation for the keys used in the dictionaries.
 * If something goes wrong, then this function returns NULL.
 */
CFArrayRef CopyLaunchedApplicationsInFrontToBackOrder(void)
{    
    CFArrayRef (*_LSCopyApplicationArrayInFrontToBackOrder)(uint32_t sessionID) = NULL;
    void       (*_LSASNExtractHighAndLowParts)(void const* asn, UInt32* psnHigh, UInt32* psnLow) = NULL;
    CFTypeID   (*_LSASNGetTypeID)(void) = NULL;
    
    void *lsHandle = dlopen("/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/LaunchServices", RTLD_LAZY);
    if (!lsHandle) { return NULL; }
    
    _LSCopyApplicationArrayInFrontToBackOrder = (CFArrayRef(*)(uint32_t))dlsym(lsHandle, "_LSCopyApplicationArrayInFrontToBackOrder");
    _LSASNExtractHighAndLowParts = (void(*)(void const*, UInt32*, UInt32*))dlsym(lsHandle, "_LSASNExtractHighAndLowParts");
    _LSASNGetTypeID = (CFTypeID(*)(void))dlsym(lsHandle, "_LSASNGetTypeID");
    
    if (_LSCopyApplicationArrayInFrontToBackOrder == NULL || _LSASNExtractHighAndLowParts == NULL || _LSASNGetTypeID == NULL) { return NULL; }
    
    CFMutableArrayRef orderedApplications = CFArrayCreateMutable(kCFAllocatorDefault, 64, &kCFTypeArrayCallBacks);
    if (!orderedApplications) { return NULL; }
    
    CFArrayRef apps = _LSCopyApplicationArrayInFrontToBackOrder(-1);
    if (!apps) { CFRelease(orderedApplications); return NULL; }
    
    CFIndex count = CFArrayGetCount(apps);
    for (CFIndex i = 0; i < count; i++)
    {
        ProcessSerialNumber psn = {0, kNoProcess};
        CFTypeRef asn = CFArrayGetValueAtIndex(apps, i);
        if (CFGetTypeID(asn) == _LSASNGetTypeID())
        {
            _LSASNExtractHighAndLowParts(asn, &psn.highLongOfPSN, &psn.lowLongOfPSN);
            
            CFDictionaryRef processInfo = ProcessInformationCopyDictionary(&psn, kProcessDictionaryIncludeAllInformationMask);
            if (processInfo)
            {
                CFArrayAppendValue(orderedApplications, processInfo);
                CFRelease(processInfo);
            }
        }
    }
    CFRelease(apps);
    
    CFArrayRef result = CFArrayGetCount(orderedApplications) == 0 ? NULL : CFArrayCreateCopy(kCFAllocatorDefault, orderedApplications);
    CFRelease(orderedApplications);
    return result;
}

#include <stdio.h>
#include <CoreFoundation/CoreFoundation.h>
#include <dlfcn.h>
#include <ApplicationServices/ApplicationServices.h>


#import <Foundation/Foundation.h>

void PrintLaunchedApplications(CFArrayRef apps) {
    CFIndex count = CFArrayGetCount(apps);
    for (CFIndex i = 0; i < count; i++) {
        CFDictionaryRef appInfo = CFArrayGetValueAtIndex(apps, i);
        
        // Extract application name
        CFStringRef name = (CFStringRef)CFDictionaryGetValue(appInfo, kCFBundleNameKey);
        if (name) {
            CFIndex pid = 0;
            
            // Extract process ID (PID)
            // CFNumberRef pidNumber = (CFNumberRef)CFDictionaryGetValue(appInfo, kCFBundleExecutableKey);
            // CFNumberRef pidNumber = (CFNumberRef)CFDictionaryGetValue(appInfo, kCFBundleIdentifierKey);
            // CFNumberRef pidNumber = (CFNumberRef)CFDictionaryGetValue(appInfo, kProcessIDKey);
            // CFNumberGetValue(pidNumber, kCFNumberCFIndexType, &pid);
            
            // Print the name and PID
            NSLog(@"Name: %@", name);
        }
    }
}


#import <Cocoa/Cocoa.h>
int main(int argc, const char * argv[]) {
    @autoreleasepool {
        NSWorkspace *workspace = [NSWorkspace sharedWorkspace];
        NSArray *runningApps = [workspace runningApplications];

        printf("Public:\n");
        printf("========\n");
        for (NSRunningApplication *app in runningApps) {
			if ([app.localizedName isEqualToString:@"Emacs"] || [app.localizedName isEqualToString:@"Firefox"] || [app.localizedName isEqualToString:@"Finder"] || [app.localizedName isEqualToString:@"iTerm2"] || [app.localizedName isEqualToString:@"VeraCrypt"] || [app.localizedName isEqualToString:@"Spotify"] || [app.localizedName isEqualToString:@"Tutanota Desktop"] || [app.localizedName isEqualToString:@"Trello"] || [app.localizedName isEqualToString:@"Signal"] || [app.localizedName isEqualToString:@"TeXShop"] || [app.localizedName isEqualToString:@"thunderbird"]) {
    	        NSLog(@"Name: %@", [app localizedName]);
        	    // NSLog(@"Application ID: %@\n\n", [app bundleIdentifier]);
	        }
        }
    }

    printf("\n===========================\n\n");
    printf("Private:\n");
    printf("========\n");

    CFArrayRef launchedApplications = CopyLaunchedApplicationsInFrontToBackOrder();
    
    if (launchedApplications) {
        PrintLaunchedApplications(launchedApplications);
        CFRelease(launchedApplications); // Don't forget to release the array when done
    } else {
        printf("Error: Unable to retrieve launched applications.\n");
    }
    
    return 0;
}


// /Applications/Emacs.app/Contents/MacOS/Emacs-arm64-11
// /Applications/Firefox.app/Contents/MacOS/firefox
// /Applications/Emacs.app/Contents/MacOS/Emacs-arm64-11
// /System/Library/CoreServices/Finder.app/Contents/MacOS/Finder
// /Applications/iTerm.app/Contents/MacOS/iTerm2
// /Applications/VeraCrypt.app/Contents/MacOS/VeraCrypt
// /Applications/Spotify.app/Contents/MacOS/Spotify
// /Applications/Tutanota Desktop.app/Contents/MacOS/Tutanota Desktop
// /Applications/Trello.app/Contents/MacOS/Trello
// /Applications/Signal.app/Contents/MacOS/Signal
// /Applications/TeX/TeXShop.app/Contents/MacOS/TeXShop
// /Applications/Thunderbird.app/Contents/MacOS/thunderbird
