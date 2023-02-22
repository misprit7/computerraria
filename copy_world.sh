case $1 in
    -t|--to)
        cp ./computer.wld ~/.local/share/Terraria/tModLoader/Worlds/computer.wld
        ;;
    -f|--from)
        cp ~/.local/share/Terraria/tModLoader/Worlds/computer.wld .
        ;;
    *)
        "Supply either --to or --from to copy either from or to the Terraria world files" 
        ;;
esac        
