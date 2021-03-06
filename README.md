echo -- write arguments to the standard output
---

### Usage
echo [-n] [string ...]

### Description
The echo utility writes any specified operands, separated by single blank (' ') characters and followed by a newline ('\n') character, to
the standard output.

The following option is available:

-n    Do not print the trailing newline character.  This may also be achieved by appending '\c' to the end of the string, as is done by
iBCS2 compatible systems.  Note that this option as well as the effect of '\c' are implementation-defined in IEEE Std 1003.1-2001
(''POSIX.1'') as amended by Cor. 1-2002.  Applications aiming for maximum portability are strongly encouraged to use printf(1) to
suppress the newline character.

