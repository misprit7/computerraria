#!/bin/sh

#################################################
# Copies world to/from game files
#################################################

TARGET_DIR="$HOME/.local/share/Terraria/tModLoader/Worlds"
TO_FLAG=0
FROM_FLAG=0
MNT_FLAG=0

for arg in "$@"
do
    case $arg in
        -t|--to)
            TO_FLAG=1
            ;;
        -f|--from)
            FROM_FLAG=1
            ;;
        --mnt)
            MNT_FLAG=1
            ;;
        *)
            echo "Invalid option: $arg"
            echo "Supply either --to or --from and optionally --mnt to specify directory"
            exit 1
            ;;
    esac
done

if [ "$MNT_FLAG" -eq 1 ]; then
    TARGET_DIR="/mnt/d/computerraria"
fi

if [ "$TO_FLAG" -eq 1 ]; then
    cp $(dirname "$0")/computerraria.wld $TARGET_DIR
    cp $(dirname "$0")/computerraria.twld $TARGET_DIR
elif [ "$FROM_FLAG" -eq 1 ]; then
    cp "$TARGET_DIR/computerraria.wld" $(dirname "$0")
    cp "$TARGET_DIR/computerraria.twld" $(dirname "$0")
else
    echo "Supply either --to or --from to copy either to or from the Terraria world files"
    exit 1
fi
