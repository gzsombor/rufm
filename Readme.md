# Rufm
## **Ru**st **F**ile **M**anager

Rufm is a file manager written in rust with a responsive terminal user interface (tui) and basic functionality for linux.
It probably works on MacOSX as well, but I haven't tested it yet. If you tried it, let me know if it works for you.

![Rufm preview](images/preview.png "Preview")

#
## Content

* Installation
* Usage
* Configuration
* Dependencies

# 
## Installation

Because rufm was built with rust the installation and compilation isn't very complicated!
And because I haven't used many other crates it should not be too slow, probably like one or two minutes.

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
## Usage

NOTE: If you use an older version, you probably have to update your configuration file.

This project is in it's early states. Currently there aren't many features avaible.
I hope this is going to change in the near future.

To use rufm just type "rufm" into the terminal and it should start (only if you copied it somewhere in your PATH).

You can use flags to change the behaviour.
If you want to use a custom path for your configuration file, use "-c".
Example: use the configuration file located at your_current_director/config/config.ini
``` bash
rufm -c config/config.ini
```
If you want to start rufm in a different directory, use "-d".
Example: start rufm in your home directory
``` bash
rufm -d ~
```
Use the help menu if you forget something:
``` bash
rufm --help
```

Because rufm is based on a terminal user interface you can only navigate while using keyboard shortcuts. In a nutshell, capital letters stand for actions, non-capital letters for navigation. Here's a list of all keybindings:

- D => Deletes all selected files / directories + the current selected one
- C => Copies all selected files / directories + the current selected one
- P => Pastes all copied files / directories
- R => Renames a file or directory

- F => Switch to the favourites tab
- / => Switch to search

- Tab => Change the sorting style (normal and length)
- Space => Select an element (to delete / copy multiple files)

All of the top keybindings can be changed in the configuration file.

- Esc => Exit and switch back to the filelist
- Enter => What enter normally does!
    - open file
    - apply search
    - select favourite from the favourites list

Use the vim or the four arrow keys for navigation.
h or the left arrow key goes to the previous directory, l or the right arrow key into the currently selected one.
If you want to quit use q.

#
## Configuration

To configure rufm you have to edit or create the configuration file located at "~/.config/rufm/config.ini". Alternatively as shown in the usage section, you can use the "-c" flag to point to another configuratin file. You can find the default configuration file in the config folder. Comments for easier use are provided.

The file should look like this:

``` toml
## in rgb form
## -> [red, green, blue]

## border colors
[borders]
search = [159, 222, 11]
info = [67, 222, 134]
filelist = [198, 40, 222]
preview = [222, 20, 111]
favourites = [222, 146, 60]

## to use no colors, 
## just comment them out
[highlights]
border = [132, 150, 232]
text.fg = [132, 150, 232]
# text.bg = [0, 0, 0]
symbol = ">"

## favourites
[favourites]
names = ["Root", "Home", "Testing"]
paths = ["/", "~", "~/projects/testing"]

## keybindings
## to use the default keybindings, just comment them out
[keys]
rename = "R"
copy = "C"
paste = "P"
delete = "D"
search = "/"
sort = "\t"
favourites = "F"
select = " "

## other stuff
[other]
##  basic information on startup
# startup_info = true
## cmd that opens the selected file
## if commented out, the $EDITOR is used + filename
## else the open_cmd + filename
# open_cmd = "code"
```

There a few things you have to consider:
The arrays names and paths have to be the same length.
You can comment out some of the commands to use the default or no values. If commenting out is an option, it is noted in the comments.


#
## Notes

Now you know everything you need to get started. I hope you have fun and you can use my tool effectively. 
If you have suggestions or if you'd like to contribute, let me know through an issue.

# 
## Dependencies

To draw the tui I used:
- tui-rs => https://github.com/fdehau/tui-rs
- termion => https://github.com/redox-os/termion

To read from the configuration file I used:
- toml => https://github.com/alexcrichton/toml-rs
- serde and serde_derive => https://github.com/serde-rs/serde
