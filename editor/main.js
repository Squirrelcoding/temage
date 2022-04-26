// Note: a lot of this code was just stolen, so im just as confused as you.

// Import CSS and file-saver dependency used to save the file
import './style.css'
import {saveAs} from "file-saver"


// Um, initiate the elements? I havent written vanilla js code in a while.
const container = document.querySelector('.container')
const sizeEl = document.querySelector('.size')
const color = document.querySelector('.color')
const resetBtn = document.querySelector('.btn')

// Get the size of the image

let size = Number(sizeEl.value);

// Initiate a new buffer with the size^2 plus one. The +1 is used to make space for the first buffer, which contains the size of the image
let arr = new Uint8Array((size * size) + 1);
arr[0] = size;

// idk what draw does
let draw = false


function populate(size) {
    
    // does stuff
    container.style.setProperty('--size', size)

    // Loops through every pixel, but starts at 1, cause the first pixel is used to store the size of the image
    for (let i = 1; i < (size * size) + 1; i++) {

        // Initially set all the pixels to 0
        arr[i] = 0;

        // Do stuff
        const div = document.createElement('div')
        div.classList.add('pixel')


        // Event listener for when a pixel is drawn on
        div.addEventListener('mousedown', function (e) {
            var rect = container.getBoundingClientRect();

            //Get the mouse position, *not* the coordinate
            let mousePosition = {
                x: e.clientX - rect.left,
                y: e.clientY - rect.top
            };

            // This messy line converts a 2D coordinate into a position on an array
            let j = (Math.ceil(mousePosition.x / (rect.width / size)) - 1) + (size * (Math.ceil(mousePosition.y / (rect.height / size)) - 1))

            // Set the array index to the value of the color currently selected. Can you guess why its j + 1 and not j?
            arr[j + 1] = colorToCode(color.value);

            // Visually set the color of the pixel to the color currently selected
            div.style.backgroundColor = color.value
        })


        // This is so that the user can draw by clicking and holding the mouse
        div.addEventListener('mouseover', function (e) {

            // idk what draw does
            if (!draw) 
                return
            
                var rect = container.getBoundingClientRect();
            

            // Get the mouse position, *not* the coordinate 
            let mousePosition = {
                x: e.clientX - rect.left,
                y: e.clientY - rect.top
            };

            // Convert 2D coordinate to a position on the array
            let j = (Math.ceil(mousePosition.x / (rect.width / size)) - 1) + (size * (Math.ceil(mousePosition.y / (rect.height / size)) - 1))

            // Set the array index to the value of the color currently selected.
            arr[j + 1] = colorToCode(color.value);


            // Visually set the color of the pixel to the color currently selected
            div.style.backgroundColor = color.value
        })

        // idk what this does, but it looks important 
        container.appendChild(div)
    }
}


// These two event listeners do... stuff.
window.addEventListener("mousedown", function (e) {
    draw = true
})
window.addEventListener("mouseup", function () {
    draw = false
})


// Reset and/or draw a new image to draw on 
function reset() {

    // Reset the array
    arr = new Uint8Array((size * size) + 1);

    // Set the first byte to the width of the image
    arr[0] = Number(size);


    // Does something
    container.innerHTML = ''

    // Run code to do stuff to the grid
    populate(size)
}


// When the reset button is clicked it runs the reset() function
resetBtn.addEventListener('click', reset)


// Does something,
sizeEl.addEventListener('keyup', function () {
    size = sizeEl.value
    reset()
})

// Takes a hex value for a color and returns a value from 0 inclusive to 7 inclusive.
function colorToCode(color) {
    switch (color) { 
        
            // Red
        case "#cd3131":
            return 1;

            // Green
        case "#0dbc79":
            return 2;

            // Blue
        case "#2472c8":
            return 4;

            // Yellow
        case "#e5e510":
            return 3;

            // Purple
        case "#bc3fbc":
            return 5;

            // Cyan
        case "#11a8cd":
            return 6;

            // White
        case "#e5e5e5":
            return 7;

            // Black / Eraser
        case "#3d3d3d":
            return 0;

        default:
            return null;
    }
}


// Creates a blob from the array, and saves it to the disk with some library.
export function createFile() {
    var blob = new Blob([arr], {type: "application/octet-stream"});
    saveAs(blob, "image.tm");
}


// Make it available to the HTML file, its hacky, but it works
window.createFile = createFile;