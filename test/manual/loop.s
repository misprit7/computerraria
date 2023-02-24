.global _start
.text
# bin read /home/xander/dev/terraria/computerraria/test/manual/loop.txt

_start:      
xor x1, x1, x1
xor x2, x2, x2
xor x3, x3, x3
xor x4, x4, x4
addi x1, x1, 1
addi x2, x2, 3
addi x4, x4, 5
loop:
add x1, x1, x2
addi x3, x3, 1
blt x3, x4, loop
sw x1, 100(x0)
lui x5, 0x1
addi x5, x5, 2046
addi x5, x5, 2046
sw x2, 0(x5)
