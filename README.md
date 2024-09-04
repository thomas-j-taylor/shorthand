# Shorthand
## Example
The following command will let you press one of the key sequences lf, ht, or UU, to select and run a command from ```lf```, ```htop```, or ```sudo pacman -Syu``` respectively.
```
eval $(echo 'lf lf\nht htop\nUU sudo pacman -Syu' | shorthand)
```
## Description
Shorthand is a tui program which allows selection from a list of strings, much like dmenu or fzf. The difference is that shorthand is primarily for implementing memorable, vim-styled keybindings. The reason why I created this is because I had developed a habit of absent-mindedly opening lf immediately after opening my terminal. The keybinding system in lf was the most convenient way for me to navigate to a given directory and jump into files without creating any extra mental work or unneccessary typing, it also allowed me to keep track of hard to remember commands which I occassionaly used. I felt that this functionality ought to exist without a file-manager attached.

## Installation
Assuming that you have cargo installed, you can install shorthand using the command:
```
cargo install --git https://github.com/thomas-j-taylor/shorthand.git
```

## Usage
The following script shows how shorthand would typically be used, placed at location such as '~/.local/bin/shorthand-keys.sh'. Such a script would be called via a keybinding using the the command ```eval $(~/.local/bin/shorthand-keys.sh)```.

```
#!/usr/bin/env bash

COMMENTS_AND_BLANK_LINES='^\s*$\|^\s*#'

# Press ee to edit this file
EDIT_KEYBINDINGS="ee $EDITOR $(readlink -f $0)"

select_keybinding() {
    { cat $1; echo $EDIT_KEYBINDINGS; } | grep -v "$COMMENTS_AND_BLANK_LINES" | shorthand
}

select_keybinding <<'EOF'
# change directories
gh cd ~
gr cd /
gc cd ~/.config
gd cd ~/Documents
gl cd ~/Downloads
gs cd ~/.local/bin
g/ cd $(find ~/Documents ~/.config -maxdepth 1 -mindepth 1 -type d | fzf)

# edit files
e/ selection="$(find ~ ~/Documents ~/.local/bin ~/.config/* -maxdepth 1 -mindepth 1 -type f | fzf)"; [[ -n "$selection" ]] && nvim "$selection"

# tmux shortcuts
t| tmux split -h
t- tmux split -v

# network commands
nwl nmcli device wifi list
nwc nmcli -g SSID device wifi list | fzf | xargs -I {} nmcli device wifi connect {}
nwa ip addr
nwn ip neig

# disk usage
uf df -h . | awk 'NR==2{print "Remaining space: "$4}'
u. du -hs . | awk '{print "Size of current directory: "$1}'
EOF
```

Adding the following line to the .zshrc file would enable access the shortcuts using ctrl+g as a leader key:
```
bindkey -s '^g' 'eval $(~/.local/bin/shorthand-keys.sh)\n'
```
