# rbtex
A small Rust tool that renders retro terminal style text animations into a video file.

It parses a simple markup language, renders text frame by frame and merges every frame into an MP4 video using ffmpeg.



## Project Structure
```
.
├─ assets/
|  └─ your_favorite_font.ttf      # the font to use
|
├─ input/
|  └─ input.txt                   # input script
|
├─ output/
|  ├─ frames/                     # temporary frames (removed automatically)
|  └─ output.mp4                  # final video
|
├─ src/
|  ├─ main.rs
|  ├─ parser.rs
|  └─ renderer.rs
|
├─ Cargo.lock
├─ Cargo.toml
├─ LICENSE
└─ README.md
```

## Requirements

- Rust
- ffmpeg available in `PATH`


## Script Syntax

The renderer parses text character by character, with commands delimited by `§`

### Commands

- **Pause**
Pauses for a variable time.
```
§p:<time_to_wait>§
```
Example:
```
§p:30§
```
Pauses for 30 units (one unit is the time it takes to print a single character).


- **Instant text**
Renders the entire text in one unit.
```
§i:<text>§
```
Example:
```
§i:This text is very fast§
```
It is impossible to nest commands in an instant text.


- **Color change**
Changes the color for the following text.
```
§c:<color_name>§
§c:r,g,b§
```
Example:
```
§c:red§this text is red
this one also §i:even this one§
§c:reset§now we go back to the initial color
```
Built-in colors:
- red   : (255, 000, 000)
- green : (000, 255, 000)
- blue  : (000, 000, 255)
- reset : (255, 255, 255)


## Configuration (in code)
Currently configured in main.rs

```rust
let width = 1440 / 2;
let height = 1080 / 2;

let font_path = "assets/your_font.ttf";
```

for now, FPS is fixed at 60, with each character lasting 4 frames


## Future ideas
- Maybe adding a config file to easily change format, file paths, fps, ...
- CLI Arguments for input, output
- More built-in colors
- More commands (repeat a char x times, clear the screen, move on the screen, ...)
- The ability to nest commands


## License

MIT