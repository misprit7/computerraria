#!/bin/sh

#################################################
# Copies world to/from game files
#################################################

case $1 in
    -t|--to)
        cp $(dirname "$0")/computer.wld ~/.local/share/Terraria/tModLoader/Worlds/computer.wld
        ;;
    -f|--from)
        cp ~/.local/share/Terraria/tModLoader/Worlds/computer.wld $(dirname "$0")
        ;;
    *)
        "Supply either --to or --from to copy either from or to the Terraria world files" 
        ;;
esac        
