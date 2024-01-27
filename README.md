# Shorthand
## Example
The following command will let you press one of the key sequences lf, ht, or UU, to select and run a command from ```lf```, ```htop```, or ```sudo pacman -Syu``` respectively.
```
eval $(echo 'lf lf\nht htop\nUU sudo pacman -Syu' | shorthand)
```
## Description
Shorthand is a tui program which allows selection from a list of strings, much like dmenu or fzf. The difference is that shorthand is primarily for implementing memorable, vim-styled keybindings in the most general possible way. The reason why I created this is because I had developed a habit of absent-mindedly opening lf immediately after opening my terminal. The keybinding system in lf was the most convenient way for me to navigate to a given directory and jump into files without creating any extra mental work or unneccessary typing, it also allowed me to keep track of hard to remember commands which I occassionaly used. I felt that this functionality ought to exist without a file-manager attached, in the most flexible possible way.
