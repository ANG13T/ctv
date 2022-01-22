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
    -e, --env        Show all ENV variables
    -V, --version    Prints version information
    -l, --layer <layer>        Sets tree layer limit
    --set-env <set-env>    Set custom ENV Variable via CLI

## Setting ENV variable via CLI
```bash
 --set-env <set-env> 
```
### <set-env> format
```bash
"<env-name>=<env-value>"
```

### Example
```
ctv --set-env "DIR_COLOR=WHITE"
```

## Customization
The .env contained within ctv allows you to customize the apperance of your tree display!

#### Configurable ENV variables
