# chip8r

My project of making a chip8 emulator with rust.

| short | option     | description                                                                        |
|-------|------------|------------------------------------------------------------------------------------|
| -r    | -rom       | path to a chip8 rom                                                                |
|       | --chip8    | start with original chip8 quirks (vf-reset+memory+clipping)                        |
| -v    | --vf-reset | quirk: jumping resets the VF flag in instructions 8XY1, 8XY2, 8XY3.                |
| -m    | --memory   | quirk: I gets incremented by X in instructions FX55, FX65.                         |
| -c    | --clipping | quirk: sprites are clipping instead of wrapping at screen bottom.                  |
| -s    | --shifting | quirk: VX shifts itself instead of shifting from VY in instructions 8XY6 and 8XYE. |
| -j    | --jumping  | quirk: jumps to NNN + VX instead of NNN + V0 in instruction BNNN.                  |
| -h    | --help     | Print help                                                                         |
| -V    | --version  | Print version                                                                      |
