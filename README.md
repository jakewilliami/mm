# Mischeif Managed

## Description

Mischeif Managed (`mm` for short) is a desktop application for macOS to toggle open applications.  Think [Aero Peek](https://www.wikiwand.com/en/Windows_Aero)/[App Exposé](https://www.wikiwand.com/en/Expos%C3%A9_%28Mac_OS_X%29).  This is the kind of app you'd want to bind to a hotkey for ease of use.  Relevant discussion [here](https://superuser.com/q/36504).

## Quick Start

### Building the Application

#### Rust

```bash
$ cd src/probe/rs
$ ./build.sh
$ ./mm
```

#### Objective-C

```bash
$ cd src/probe/
$ ./build.sh
$ ./main
```

### Keyboard Shortcut

You can set up a new service through Automator with a keyboard shortcut as described [here](https://apple.stackexchange.com/a/40887/366960).

When prompted for a document type in Automator, choose Quick Action.  Ensure the service does not wait for any input.  I have bound this to <kbd>⌘ + \</kbd>.

## Note on Development

I am well aware that this functionality is not only something that already somewhat exists (exposé), but also likely exists in third-party forms.  However, the reason I am developing it myself is because I want to.  I really enjoyed watching the streams on the development of [boomer](https://github.com/tsoding/boomer), and was inspired by the idea of writing a genuinely useful desktop app for myself.

Initial research into the development of this application uses one of the languages closest to macOS: Objective C.  The end goal is to implement this in Rust (because why not).  There may be intermediate implementations (e.g., Julia) in the process of implementing this, to get used to the FFI for Cocoa/Core Foundation.
