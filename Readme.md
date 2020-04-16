# Rufm - **Ru**st **Fi**le **Ma**nager

Rufm is a file manager written in rust with a responsive terminal user interface (tui) and basic functionality for linux.

<img src="images/fullsize.png" alt="Rufm full-sized"
	style="float: left; margin-right = 1%" width="450" height="300" />
<img src="images/smallsize.png" alt="Rufm small-sized"
	width="450" height="300" />

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

NOTE: If you use an older version, you probably have to update your configuration file.

This project is in it's early states. Currently there aren't many features avaible.
I hope this is going to change in the near future.

To use rufm just type "rufm" into the terminal and it should start (only if you copied it somewhere in your PATH).

Use the help menu for more information and avaible flags.
``` bash
rufm --help
```

To configure rufm you have to edit or create the configuration file located at "~/.config/rufm/config.ini". Alternatively you can use the "-c" flag to point to another configuratin file. You can find the default configuration file in the config folder.
The default should look like this:

``` toml
# in rgb form
# -> [red, green, blue]

# border colors
[borders]
search = [159, 222, 11]
info = [67, 222, 134]
filelist = [198, 40, 222]
preview = [222, 20, 111]
favourites = [222, 146, 60]

# to use no colors, 
# just comment them out -> optional colors
[highlights]
border = [132, 150, 232]
text.fg = [132, 150, 232]
# text.bg = [0, 0, 0]

# favourites
[favourites]
names = ["Root", "Home"]
paths = ["/", "~"]

# keybindings
# to use the default keybindings, just comment them out
[keys]
rename = "R"
copy = "C"
paste = "P"
delete = "D"
search = "/"
sort = "\t"
favourites = "F"
```

Note that the arrays names and paths have to be of the same length.

Keyboard shortcuts:

Capital letters stand for actions, non-capital letters for navigation.

- D => Deletes a file or directory (with confirmation prompt)
- C => Copies a file or directory
- P => Pastes a copied file or directory
- R => Renames a file or directory

- F => Switch to the favourites tab
- / => Switch to search

- Tab => Change the sorting style (normal and length)

All of the top keybindings can be changed in the configuration file.

- Esc => Exit and switch back to the filelist
- Enter => What enter normally does

h, j, k, l (vim keys) or the four arrow keys can be used for navigation.

# 
## Dependencies

To draw the tui I used:
- tui-rs => https://github.com/fdehau/tui-rs
- termion => https://github.com/redox-os/termion

To read from the configuration file I used:
- toml => https://github.com/alexcrichton/toml-rs
- serde and serde_derive => https://github.com/serde-rs/serde

#
## Other stuff

If you like it, let me know what you would like to have implemented next.
I personally thought of a config setting, which allows you to remap keys. Well you can already do that. Border colors are here as well.
Next there is coming syntax highlighting in the preview.
Have fun!
