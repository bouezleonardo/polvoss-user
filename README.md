# POLVOSS user programs
## Read bytes from ELF file
`xxd -i file.elf`

## C programs compilation
### Freestanding binary
`riscv64-unknown-elf-gcc \
    -march=rv32i \
    -mabi=ilp32 \
    -ffreestanding \
    -fno-pic \
    -fno-pie \
    -nostdlib \
    -Wl,-T,user.ld \
    -o file.elf \
    file.c \
    -lgcc`
  
## Rust programs compilation
### Raw binary
`rustc \
    --target riscv32i-unknown-none-elf \
    -C panic=abort \
    -C relocation-model=static \
    -C link-arg=-Tuser.ld \
    -o file.elf \
    file.rs`
