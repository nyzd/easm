# Easm
EVM assembler written in rust

# Usage
`./easm test.easm`

test.easm content:

```assembly
PUSH1 0x80
PUSH1 0x40
MSTORE
```

should return `6080604052`
