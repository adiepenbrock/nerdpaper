# nerdpaper

<img src="https://raw.githubusercontent.com/adiepenbrock/nerdpaper/main/examples/example_square.png" alt="Generated example image by nerdpaper" width="42%" align="right" />

nerdpaper is a CLI tool designed for generating images composed of randomly colored cells, each adorned with icons from a configurable icon font. Currently, nerdpaper exclusively supports predefined Monokai colors, with plans to support custom colorschemes in the future.

<br clear="right"/>

## How to use

Arguments:

| Argument | Description                                         |
| -------- | --------------------------------------------------- |
| `width`  | Desired width of the picture                        |
| `height` | Desired height of the picture                       |
| `num`    | Desired number of cells                             |
| `font`   | Path to the icon font                               |
| `output` | Path and name of the file to save the picture to    |
| `icons`  | List of icon glyphs to choose between for the cells |


Generate an image using the free [FontAwesome](https://fontawesome.com/) icon font. This example creates an image with a width of 1024px, a height of 768px, and 10 cells, with each cell randomly choosing between two icons:
```sh
nerdpaper --width 1024 --height 768 --num 10 --font ~/path/to/Font\ Awesome\ 6\ Free-Solid-900.otf --output ~/path/to/my_picture.png --icons  --icons 
```

## License
Licensed under the [MIT License](./LICENSE.md).
