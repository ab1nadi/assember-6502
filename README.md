# assembler-6502
A basic assembler for the 6502 processor. I eventually plan to use this with my 6502 emulator. 
Has built in syntax analysis, for easy detection of  labels, directives, opcodes, or operands 
that don't exist or aren't implented. 


## Using the assembler

clone the repository and do: 
```bash
cargo run -- assemblyfile.as  output.a
```
The assembler does expect 2 arguments first the input file and second the output file. 
Alternatively you could just use the binary, but I'll let whoever wants to use this
compile it themselves. 

## Label Expressions & Label Variables
The assembler supports label expressions and label variables. Pretty much anywhere a label can be placed, you can add another label to it. You can add and subtract from it. You can multiply and divide it. 

```assembly
two = 2                     ; label variable, instead of pointing to memory it just holds a value

three = two +1              ; 

LDA #($f1+12)/2 * three     ; Loads mmediate 1 byte value into a register
```

## Directives 
I've currently only implented 2 directives .ORG and .BYTE

### .ORG
This directive simply sets the byte that all labels will be relative to from that point
forward in the assembly file. 

```assembly
.ORG $FF
```

### .BYTE
This directive will put basically whatever you want in a specific memory location. Strings, Chars, 2 byte numbers, labels (2 byte numbers) can be stored as well as 1 byte numbers. 

```assembly
.BYTE 'a', '1', $ff, $FFFF, 123456, 255, 'a string', label
```
 
