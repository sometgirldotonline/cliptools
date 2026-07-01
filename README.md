# cliptools
work in progress lol

cliptools takes text on your clipboard, and figures out actions that can be taken

also happens to be my first ever rust project! i have quite enjoyed rust and i think for larger projects its going to replace c/c++ and python for me!

# requirements
wayland linux x86_64

this almost certainly wont work on x11 but i havent tried, the library i use is designed for wayland

also idk if this only works in wlroots or sumn, i use niri and i havent tried anything else

# how to use
`git clone` and run `cargo run`. the binary will end up in `./target/debug/cliptools`

you can also grab the binary from releases

to open using a shortcut, use your desktop/window managers options to bind, personally i bind it to alt+c

# things that arent implemented
any unimplemented actions *may* show a button, but it wont do anything, afaik, i have not tried



## Actions
for telephony options, it should accept any phone number, if theres no area code it will add your local one, if the "Open In Telephony App" button doesnt do anything ensure you have some kind of telephony app, like KDE Connect to use your actual phone or a voip client of some kind that uses normal numbers
- [x] Slack User ID opening
- [x] Discord User ID opening
  - This in theory can open the desktop app if your client bound the discord:// URI
- [x] Opening Emails
- [ ] Opening Bluesky/ATProto stuff
- [ ] Opening Fedi handles
- [x] Opening Matrix handles
- [x] Calling number using phone app of choice (using `tel:`)
- [x] Opening number in Whatsapp or Telegram
- [x] Opening URIs
- [ ] Opening Files and Folders (in file manager, terminal, and default app)
- [x] Opening repos in Github if you just have the user/repo syntax copied
  - I plan to add a setting to add more git services to the list
- [ ] Cloning Git Repos using same syntax as above or if you have the full ssh/https git url
- [ ] General actions
  - [ ] Spellcheck
  - [ ] Asking text to local Ollama model
  - [ ] Searching Web
  - [ ] Copying cleaned up (trimming etc)
  - [ ] Translating (shortcut to google translate)
  - [ ] QR Code
  - [ ] Encoding (b64, uri, etc)
  - [ ] Changing case of text
  - [ ] Removing blank lines
  - [ ] Basic find and replace
## Infoboxes
The app will also show infoboxes like:
  - [ ] Colour info
  - [ ] Number Conversion (hex, bin, oct, dec, etc)
  - [ ] Text infos (wordcount etc)
  - [ ] Math Expressions

# AI use
pretty low, just the occasional help and refactoring to be cleaner because i'm new to rust and make bad decisions instead of finding the right way to do things, if you look in commit messages or code commits i typically say ai usage there
