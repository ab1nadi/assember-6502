# assembler-6502
A basic assembler for the 6502 processor. I plan to use this with my 6502 emulator. 
Has built in syntax analysis, for easy detection of  labels, directives, opcodes, or operands 
that don't exist or aren't implemented. 


## Using the Assembler
* Clone the repository.

* Compile and run: 
```bash
cargo run -- assemblyfile.asm  output.a
```
The assembler does expect 2 arguments first the input file and second the output file. The file extensions can be whatever, I haven't constrained it. Just make sure the first argument is a text file with assembly in it. The output file will have object code in it regardless of its name and extension.

## Some Quirks and Future Innovations 
* At the moment the assembler just outputs a raw object file, with no headers. At some point I want to implement the o65 6502 binary relocation format for my object code. But in the meantime this assembler fits my purposes. 
* There is no linker. In the future it might be nice to include more than one file in an assembly, so that is something I might look into. 


## Label Expressions & Label Variables
The assembler supports label expressions and label variables. Pretty much anywhere a label can be placed, you can add another label to it. You can add and subtract from it. You can multiply and divide it. Something to note is that you can mix 1 byte numbers and 2 byte numbers in expressions. But if you do, the whole expression will cast up to a 2 byte number. This can goof with you if you are trying to use an instruction that expects a 1 byte number and you give a 2 byte number. 


```assembly
two = 2                         ; label variable, instead of pointing to memory it just holds a value

three = ((two + (1 + 1)) * 3)/3 ; 

LDA #($f1+12)/2 * three         ; Loads immediate 1 byte value into register A
```

## Directives 
I've currently only implented 2 directives .ORG and .BYTE

### .ORG
This directive sets the byte that all labels will be relative to from that point
forward in the assembly file. 

```assembly
.ORG $FF
```

### .BYTE
This directive will put whatever you want in a specific memory location. Strings, Chars, 2 byte numbers, 1 byte numbers, and labels.

```assembly
.BYTE 'a', '1', $ff, $FFFF, 123456, 255, 'a string', label
```
 
