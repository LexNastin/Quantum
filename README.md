# Quantum

## What is it?

Quantum is an open source presentation program for churches, making it easy to (soon™) be able to present psalms, scriptures, slideshows, and videos.

## What makes Quantum special?

Well, our main "selling point" per se, is the fully web based controls, in combination with cross platformness, allowing you to start the program on something such as a Raspberry Pi, which can be found extremely cheap online, and be able to control from any device! Another thing, is that it is more modern, with a better, albeit equally intuitive, UI design, and user centred design; anybody can suggest changes, directly gives us code, or tell us about bugs/wanted features, and we will work towards improving the user experience.

## Why'd you make this?

My main problem was that EasyWorship was the only notable program I could use that I liked, but ever since making the <sub>(glorious)</sub> switch to Linux, I've had to carry 2 laptops with me to church, as we don't really have a permanent setup there at the moment, and well... EasyWorship is also paid, and closed source, and the only open source solutions didn't really suit me. Interfaces too cluttered, no EasyWorship 6 import, bad/no remote controls, and the problems go on. So here we are!

Also, a choice I made a little after properly beginning work on the project was switching from Python -> Rust, as

1. I wanted a bit of a challenge, as it's a new langauge to me, and
2. It just seems like a nice langauge with lots of nice features, making me want to switch to Rust as a main language

# Upcoming Features List

- [ ] Full Presentation
  - [ ] Something (at least plain text)
  - [ ] Psalms
  - [ ] Scriptures
  - [ ] PowerPoint Slideshows
  - [ ] Videos
- [ ] Remote Controls
  - [ ] API
    - [ ] Scheduling
      - [ ] Creation
      - [ ] Import
      - [ ] Export
    - [ ] Controlling Schedules
    - [ ] Downloading Schedules
    - [ ] Uploading Schedules
    - [ ] Multi User
  - [ ] Basic UI
  - [ ] Modern UI Update
    - [ ] Customizable Layout With Presets (such as EasyWorship layouts)
  - [ ] Extras
    - [ ] Scripting
- [ ] Import From Other Programs
  - [ ] EasyWorship 6 (will be easy due to format of schedule literally being a zip file containing a mini database, and the song database is just plain sqlite3 containing rtf files essentially)
  - [ ] EasyWorship 7 (same remarks as EW6)
  - [ ] EasyWorship 2007
  - [ ] EasyWorship 2009
  - [ ] VisioBible
  - More To Come
- [ ] Database Synchronised With Main Server
  - [ ] Psalms
  - [ ] User Preferences
  - [ ] Schedules
- [ ] Extra Features
  - [ ] Plugin Support (will probably work on Python using sockets)
  - [ ] Customizable NDI Stream Output

---

P.S. I'm planning to work on this over the summer as my main project, but there is not gurantee that I'll be working on it. If you want more progress, you can message me on Discord (ArchGryphon9362#6132) to remind me to work on this :sweat_smile:.
