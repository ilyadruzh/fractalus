import * as wasm from "rustfractals";

// let result = wasm.wasmdraw(100, 100, 100);

const var_2 = (result) => {

    let counter = 0;
    let tmpArr = [];
    let resultArr = [];

    for (let value of result) {
        counter++;
        tmpArr.push(value);

        if (counter % 5 === 0) {
            resultArr.push(tmpArr);
            tmpArr = []
        }
    }
    resultArr.push(tmpArr);

    return resultArr;
};

// const var_1 = (result) => {

//     var res = [];

//     for (let i = 0; i <= result.length; i += 5) {
//         res.push(result.slice(i, i + 3));
//     }
// }

let fractal_array = var_2(wasm.wasmdraw(500, 500, 500));

// Get the canvas and context
// var canvas = document.getElementById(".fractal");
// var context = canvas.getContext("2d");

// // Width and height of the image
// var imagew = canvas.width;
// var imageh = canvas.height;

// Image Data (RGBA)
// var imagedata = context.createImageData(imagew, imageh);

const componentToHex = (x) => {
    var hex = x.toString(16);
    return hex.length == 1 ? "0" + hex : hex;
}

const rgbToHex = (r, g, b) => {
    return "#" + componentToHex(r) + componentToHex(g) + componentToHex(b);
}

var canvas = document.querySelector('.fractal');
var ctx = canvas.getContext('2d');

// var width = canvas.width = window.innerWidth;
// var height = canvas.height = window.innerHeight;

fractal_array.forEach(function (el) {
    let color = rgbToHex(el[2], el[3], el[4]);
    ctx.fillStyle = color;
    ctx.fillRect(el[0], el[1], 1, 1);
})
