# ctv - configurable tree view

A highly configurable tree view visualizer CLI tool written in Rust!

<img src="./media/ctv_preview.png" width="750" title="CTV Preview Image">


## What does ctv do?

- Visualize your file hiearchy in a tree view
- Customize the apperance of your tree
- Display custom file information (permissions, time, user, etc)
- Personalize tree color and text styling

## Installation
``` bash
cargo install ctv
```

## Using ctv
``` bash
ctv <flags> <directory_path>
```
## Flag Options
    -i, --ct         Shows the file created time instead of the file modified time
        --help       Prints help information
    -h, --short      Uses short format
    -c, --config       Show all config variables
    -V, --version    Prints version information
    -l, --layer <layer>        Sets tree layer limit

## Customization
The config.toml file located within the ctv project directory (dev.ctv.ctv) allows you to customize the apperance of your tree display!
The dev.ctv.ctv directory and config.toml file must be manually created to start configuring your tree view! 

#### Default Configurable config.toml variables
```
file_size_position="1"
file_owner_position="2"
file_perms_position="3"
file_time_position="4"
file_extension_position="-1"
file_time_color="LIGHTCYAN"
file_size_color="BLUE"
file_owner_color="LIGHTMAGENTA"
file_perms_color="BLUE"
file_extension_color="YELLOW"
dir_color="BLUE"
symlink_color="LIGHTMAGENTA"
path_color="WHITE"
pipe_color="YELLOW"
chard_color="YELLOW"
blockd_color="LIGHTGREEN"
socket_color="LIGHTRED"
read_color="LIGHTGREEN"
write_color="LIGHTRED"
execute_color="LIGHTGREEN"
dash_color="LIGHTBLACK"
dir_name_style="NORMAL"
file_name_style="NORMAL"
file_time_style="BOLD"
file_size_style="BOLD"
file_owner_style="NORMAL"
file_perms_style="BOLD"
dir_name_color="LIGHTRED"
file_name_color="LIGHTRED"
file_extension_style="ITALIC"
file_time_format="%m-%d-%Y::%H:%M:%S"
file_time_type="CREATED"
tree_layer_limit="3"
spacing="0"
pipe="│"
elbow="└──"
tee="├──"
pipe_prefix="│"
space_prefix=" "
show_file_metadata="TRUE"
show_dir_metadata="TRUE"
show_short="FALSE"
```

### Font Color and Styling Options

#### Available Colors
```bash
BLACK
BLUE
CYAN
GREEN
LIGHTBLACK
LIGHTBLUE
LIGHTCYAN
LIGHTGREEN
LIGHTMAGENTA
LIGHTRED
LIGHTWHITE
LIGHTYELLOW
MAGENTA
RED
WHITE
YELLOW
```

#### Available Font Styles
```bash
BOLD
DIMMED
ITALIC
UNDERLINE
BLINK
REVERSE
HIDDEN
STRICKEN
NORMAL
```