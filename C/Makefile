# Use: make [TARGET] FILE=<file>
#
# Build Targets:
#      asm   - Builds <FILE>.asm assembly file
#      build - Builds all source files
#      bytes - Builds all source files and writes the ELF bytes in an text file 
#      clean - removes all generated files

CC      = riscv64-unknown-elf-gcc

CFLAGS  = -march=rv32i \
          -mabi=ilp32 \
          -ffreestanding \
          -fno-pic \
          -fno-pie \
          -nostdlib \
          -Iplibc

LDFLAGS = -Wl,-T,user.ld

LIBS    = -lgcc

# Source and output files
SRC = $(FILE).c
ELF = $(FILE).elf
ASM = $(FILE).asm
TXT = $(FILE).txt

# C sources from plibc
PLIBC_SRC = $(wildcard plibc/*.c)

# Default target
.PHONY: all asm build bytes clean

all: build

# Generate assembly
asm: $(ASM)

$(ASM): $(SRC)
	$(CC) $(CFLAGS) -S $< -o $@

# Compile and link executable
build: $(ELF)

$(ELF): $(SRC) $(PLIBC_SRC) user.ld $(wildcard plibc/*.h)
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $(SRC) $(PLIBC_SRC) $(LIBS)

# Generate byte array
bytes: $(TXT)

$(TXT): $(ELF)
	xxd -i $< > $@

# Remove generated files
clean:
	rm -f *.asm *.elf *.txt
