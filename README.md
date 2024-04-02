# "Enjoy" application

## Why does this exist?

I wanted to build an application to intrusively notify me to stop working and "go and enjoy life". I have a tendency to keep working when I should just call it a day. Also, I wanted a reason to learn some Rust UI frameworks and also to make sure my Rust language skills do not get rusty as I don't use it anymore in my day job.

## WIP

This is still a work-in-progress and more updates will come.
Currently it's in a proof of concept phase where I feel confident this will work. Everything now needs to be refactored.
I still don't know I will need to create some kind of "daemon" for this. First I need to learn about daemons.

Also, currently macOS only. I succeed I'll make sure this works on Windows as well. I just need to learn how to create "status bar" menus as well, as this currently depends on a macOS only `system_status_bar_macos` crate.