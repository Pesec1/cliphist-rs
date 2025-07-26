### cliphist-rs

### Big thanks to this [cliphist-go](https://github.com/sentriz/cliphist) <3. It was what got me inspired to write my own clipboard manager
_Clipboard history “manager” for Wayland_ written in rust :)

- Write clipboard changes to a history file.
- Recall history with **rofi**
- Both **text** and **images** are almost full supported.
    - Copying images from browser does not work.
    - Copying text/uri-list does not work.

Requires [wl-clipboard](https://github.com/bugaevc/wl-clipboard), xdg-utils (for image MIME inference).

### Usage

#### Listen for clipboard changes

`$ wl-paste --type text --watch cliphist-rs --store`  
`$ wl-paste --type image --watch cliphist-rs --store`  
This will listen for changes on your primary clipboard and write them to the history.  
Call it once per session - for example, in your Sway config.

### TODO
 - [ ] Add max element size
 - [ ] Find out why text/uri-list does not get automatic discovery by wl-copy
 - [ ] Add config for basic things like path_to_db, max_size
 - [ ] Add flag to delete element from db
 - [ ] Add flag to wipe database
 - [ ] Add instruction for installing 
 - [ ] Add example usage

Other clipboard managers:
 1. [cliphist](https://github.com/sentriz/cliphist)
 1. [clipman](https://github.com/chmouel/clipman)
