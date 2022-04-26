# Temage
Draw and display pixel art on your terminal

## Version 1.1
Version 1.1 of temage fixes a few bugs and updates some grammatical errors in the README.md 

## Usage

### Display image
Display an image on your terminal
`temage display <path to file>`

### Open Editor
This opens the worlds crappiest image editor
`temage editor`

## Downloading
1. Download the executable on the releases section.
2. Open a shell session in the same folder in which the executable is.
3. Run `temage --help` to ensure that the executable functional
4. Make stuff!

*But what if I want to run it globally?*
[heres a link for now, we'll include instructions for this later](https://unix.stackexchange.com/questions/3809/how-can-i-make-a-program-executable-from-everywhere)


## Getting started
Assuming you have already installed it correctly, then you can proceed with the small tutorial

Run the following command to start the editor, however it should install it first, then run it again.
```sh
$ temage editor
```

It should open a new tab on your web browser and you should see something like this, the worlds-ugliest-editor-made-with-stolen-code!:

<img src="https://i.imgur.com/DEtaqMC.png" width="480px" height="270px"/>

The number on the top right is the dimensions of your image, at the moment you can only set the image to be a square. You can make it as large as 255 before something breaks (that is if your computer can even handle it). With that in mind, feel free to change it as you'd like! 

When you're done selecting a dimension, just click the `draw/reset` button on the top left and now you can see the grid! Next to the number on the right, there is a dropdown menu containing all the supported colors, so get drawing!

Once you're satisfied with your wonderful art, it's time to get it onto the terminal! Click the `create file` button next to the `draw/reset` button. Save it as `image.tm` for the sake of this tutorial, and make sure you downloaded it to the same directory as the executable. Before the last step, make sure to stop the editor with `ctrl+c` on the terminal.

Now run the command below and voilà!
```sh
$ temage image.tm
```

## How it works
Temage has a very simple way of storing images, so lets see how it works!

We start off with the legendary data structure: the array.
```
[]
```

Now lets add some stuff to it!

```
[5, 4, 4, 4, 4, 3, 4, 7, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2]
```

The numbers, what do they mean? Well, this is a temage image! Lets take a closer look. The first number contains the width of the image, this is useful for knowing when the next row of pixels begins. Excluding the first byte, the length of the array is 25. And if you passed elementary school then you that 25 is a multiple of 5. So there must be 5 rows in this image.

But what about the rest of the image? What do the rest of the numbers mean? How is an array turned into an image? Slow down, Ill answer the first two questions now. Temage images use the numbers 0 inclusive to 7 inclusive. Thats 8 whole numbers. And each number corresponds to a color. Heres a table for it:
| Numerical value | Color  |
|-----------------|--------|
| 0               | Black  |
| 1               | Red    |
| 2               | Green  |
| 3               | Yellow |
| 4               | Blue   |
| 5               | Purple |
| 6               | Cyan   |
| 7               | White  |

Lets look at the array again.
```
[5, 4, 4, 4, 4, 3, 4, 7, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2]
```
How does this 1D array get turned into a 2D image? Math! Dont worry, it's just elementary school level math. We can get the X and Y coordinates based on the position of the number on the array. I got the formulas below from [here](https://softwareengineering.stackexchange.com/questions/212808/treating-a-1d-data-structure-as-2d-grid)
```
x = i % width;    // % is the "modulo operator", the remainder of i / width;
y = i / width;    // where "/" is an integer division
```

Lets prove the formulas above, we'll take the 7 near the top left as an example. Pretending the 5 isnt on the array, its position on the array is 6, since arrays start at 0. 5 is the width, located at the beginning of the array. So lets do some math!

```
1 = 6 % 5
1 = 6 / 5 (Its actually 1.2, but we're rounding downwards)
```


We all know arrays start at 0, so its X coordinate will be 1 and Y coordinate will also be 1. In non-array coordinates that is (2, 2). We can do that for all of the other numbers (excluding the first one of course) and we get something like the one below:

```
[
    4, 4, 4, 4, 3,
    4, 7, 4, 4, 4,
    4, 4, 4, 4, 4, 
    4, 4, 4, 4, 4, 
    2, 2, 2, 2, 2
]
```

Its starting to look like an image, just remove the parentheses...
```
4, 4, 4, 4, 3,
4, 7, 4, 4, 4,
4, 4, 4, 4, 4, 
4, 4, 4, 4, 4, 
2, 2, 2, 2, 2
```

Fill in the colors, and lets see the image!
![final image](https://i.imgur.com/2288Z5t.png)
Ignore everything else, you see that little drawing in the bottom left corner? Thats what we just made! We made that little image from a list of numbers and elementary school math! Sure, it looks a little off, and there's a percent sign, (it's there for computer reasons) but it works!