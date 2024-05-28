# "Enjoy" application

## Why does this exist?

I wanted to build an application to intrusively notify me to stop working and "go and enjoy life".
I have a tendency to keep working when I should just call it a day. Also, I wanted a reason to learn
some Rust UI frameworks and also to make sure my Rust language skills do not get rusty as I don't
use it anymore in my day job.

## WIP

This is still a work-in-progress and more updates will come.
Currently it's in a proof of concept phase where I feel confident this will work.

### 🚧 Currently macOS only

Also, currently macOS only. If I succeed I'll make sure this works on Windows as well.
I just need to learn how to create "status bar" in Windows menus as well, as this currently depends
on a macOS only `system_status_bar_macos` crate.

## TODO
List of to-do items:
- [ ] v0.1: Add settings serialization logic using `serde`
- [ ] v0.1: Build Settings UI state logic based on serialized settings

- [ ] v0.2: "Improve" UI style
- [ ] v0.2: Add "apps-to-quit" logic
