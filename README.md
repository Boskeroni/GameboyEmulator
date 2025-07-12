# YAGE (Yet Another Gameboy Emulator)

Emulator for the original gameboy, with support for MBC1, MBC2, and MBC3. This was done as an A-level project in order to learn about CPU architecture and how different components communicate across a system. Due to the simplicity of the graphics rendered, `macroquad` is used.

## Usage

This project uses a justfile in order to simplify the command-line argument passing. The justfile may require the ROM's provided to be organised in a certain folder structure.

To boot from BIOS: (this will immediately exit when the BIOS has ended)

```bash
just boot
```

To play a ROM (requires file provided to be in a `rom/` folder)

```bash
just play <ROM>
```

The commands `build` and `test` are mainly just used in the development of this emulator and don't need to be used if you are just wanting to play games.

## TODO

Even though I have finished with the project, these would be the things I would add should I ever come back.

- [ ] add sound system
- [ ] add support for more obscure MBC's
- [ ] improve the timing of PPU
- [ ] allow for inter-cycle execution of subsystems.
- [ ] allow custom colors to be selected other than shades of grey
- [ ] allows for custom keybinds to be set

## Demonstration

<video width="320" height="240" controls>
    <source src="include/testing_example.mp4" type="video/mp4">
</video>

This example shows the emulator passing all of [Blargg's test ROMs](https://github.com/retrio/gb-test-roms/tree/master/cpu_instrs). These test for CPU functionality and should mean that a fully valid CPU has been built. It also partially shows MBC's working as it uses MBC1.

<video width="320" height="240" controls>
    <source src="include/zelda_example.mp4" type="video/mp4">
</video>

This example is more geared to showing the PPU working intentionally. Its playing Zelda Link's Awakening, and also shows a feature I added which allows the user to speed up and slow down the game. This was a request from one of my friend's who saw other emulators also implement it.

## Resources

- [Pandocs](https://gbdev.io/pandocs)
- [Opcodes](https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7)
- [Detailed information on the PPU](https://hacktix.github.io/GBEDG/ppu)
- [Talk which inspired this project](https://youtube.com/watch?v=HyzD8pNlpwI)
