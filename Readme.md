# Rufm - **Ru**stical **Fi**le **Ma**nager

Rufm is a file manager written in rust with a responsive terminal user interface (tui) and basic functionality.

# 
## Content

* Installation
* Usage & Configuration
* Dependencies

# 
## Installation

Because rufm was built with rust, the installation and compilation isn't very complicated!

* Clone the repo:
``` bash
git clone https://github.com/OrangeFran/rufm.git
cd rufm
```

* Start the build process with cargo:
``` bash
cargo build
```

You have now built your executable binary. To run it, change to the target/debug directory and run ./rufm:

``` bash
cd target/debug 
./rufm 
```

If you want to be able to run rufm from every directory, you have to copy it to a location which is in your $PATH variable. If you're on linux, you could use /usr/bin:

``` bash
sudo cp rufm /usr/bin/rufm
```

# 
## Usage & Configuration

