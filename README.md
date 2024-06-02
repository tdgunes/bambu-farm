# Bambu Farm 🧑🏽‍🌾

Run your own cloud service for Bambu Labs printers and unlock the full potential of LAN mode!

As of writing (September 2023) Bambu Labs has not released the source code for the `libbambu_networking.so` shared library that they link with to provide the full functionality of Bambu Studio, forked from the AGPLv3 Prusa Slicer. Until they choose to release the code on their own or somebody with deep pockets decides to sue them, you can use this project to provide your own LAN-mode "cloud" printing service. Or at least, you could, *if* you help me figure out the finishing touches like auth, bugfixes, replacing `.unwrap()`s with error handling, and general quality of life features. 👀

## How does it work?

At startup, Bambu Studio looks for a plugin called `libbambu_networking.so`, `dlopen`s it and calls into it for networking functionality. By default, it will use the proprietary version installed from Bambu Labs' servers. I don't like that, so I wrote my own. It's a drop-in replacement, and there's a Makefile in this project that will symlink the build artifacts into `$HOME/.config/BambuStudio/plugins/*` to install the FOSS plugin. Bambu Studio will then use this version instead of its own.

Unfortunately, the C++ ecosystem is full of footguns, and dynamic linking is a sham, so you can't use OpenSSL in `libbambu_networking.so` or it'll segfault. This presents a lot of issues considering Bambu Labs' use of TLS for MQTT and FTP, so I extracted all of the command and control logic into a server process and use gRPC to communicate between `libbambu_networking.so` and the server. That gRPC link will eventually be able to use TLS with `rustls` and `ring`, dodging the OpenSSL difficulties. A side-effect of this separation between client and server is that this makes the architecture scale to arbitrarily large print farms and should allow communication between clients and printers on different networks, as well as allowing the implementation of fine-grained access controls.

## What's the current status?

Currently you can control and monitor the printer's vitals, jog and home the axes and start print jobs. You can't view the camera feed yet. The implementation is very bare-bones and it's not ready for general use.

## What do I need to build/run it?

Off the top of my head, you'll need Rust/Cargo, OpenSSL (development packages), GNU Make, a C/C++ build system, a protocol buffers compiler (protoc), cURL, and a little bit of determination to fix any issues that come up. Some of my projects are very highly polished, but this is not one of them.

### Mac OS X Sonoma 14.5 Apple Silicon Specific

1. Make sure you have `cargo` installed.
2. `brew install boost msgpack-cxx protobuf`
3. `make`
4. `cd bambu-farm-client/target/aarch64-apple-darwin/debug`
5. `mkdir shared`
6. `cd shared`

(These are adapted from the Makefiles):

7. `ar -x ../libbambu_farm_client.a`
8. `gcc -shared *.o -o libbambu_networking.so -lc++`
9. `echo "" | gcc -fPIC -shared -x c -o libBambuSource.so -`
10. `rm *.o`

At this point you will have `libbambu_networking.so` and `libBambuSource.so`.

Copy these to: `/Library/Application Support/OrcaSlicer/plugins`

Start the `OrcaSlicer`!

Disclaimer: However, I couldn't make it work and observed OrcaSlicer re-downloading the proprietary libraries.

## Get involved!

If a feature you need is missing open an issue to run it by me, but it's very likely I want that feature and will accept a PR for it. A lot of the cruft in this project exists for a reason, so open an issue before you try any major refactors, or try to switch around dependencies and stuff. There's a good chance I've already tried what you have in mind and lost a few hours of my life to it.

## Any caveats?

Be aware that once you install *any* networking plugin, Bambu Studio will assume that you've installed *theirs* and will "upgrade" it whenever you install a new version of Bambu Studio by replacing it with their proprietary version without your consent. I plan on opening a bug against them for that.

## Licensing information

This project is licensed under the AGPLv3 because it contains code from Bambu Studio.
