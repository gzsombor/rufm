# Rufm - **Ru**stical **Fi**le **Ma**nager

Rufm is a file manager written in rust with a responsive terminal user interface (tui) and basic functionality for linux.

# 
## Content

* Installation
* Usage & Configuration
* Dependencies

# 
## Installation

Because rufm was built with rust the installation and compilation isn't very complicated!

* Clone the repo:
``` bash
git clone https://github.com/OrangeFran/rufm.git
cd rufm
```

* Start the build process with cargo:
``` bash
cargo build
```

You have now built your executable binary. To run it change to the target/debug directory and run ./rufm:

``` bash
cd target/debug 
./rufm 
```

If you want to be able to run rufm from every directory you have to copy it to a location which is in your PATH. You could use /usr/bin:

``` bash
sudo cp rufm /usr/bin/rufm
```

# 
## Usage & Configuration

This project is in it's early states. Currently there aren't many features avaible.
I hope this is going to change in the near future.

To use rufm just type "rufm" into the terminal and it should start, only if you copied it somewhere in your PATH.

Use the help menu for more information and avaible flags.
``` bash
rufm --help
```

To configure rufm you have to edit the configuration file located at "~/.config/rufm/config.ini".
The default should look like this:
``` toml
# rgb color codes
# -> [red, green, blue]
# to use no colors, just comment them
[colors]
border_normal = [255, 255, 255]
border_highlight = [158, 232, 255]

# highlighting of the selected text
text_highlight.fg = [158, 232, 255]

# favourites
[favourites]
names = ["Root", "Home", "Dotfiles", "Projects"]
paths = ["/", "~", "~/dotfiles", "~/projects"]
```

# 
## Dependencies

To draw the tui I used:
- tui-rs => https://github.com/fdehau/tui-rs
- termion => https://github.com/redox-os/termion

To read from the configuration file I used:
- toml 
- serde
- serde_derive