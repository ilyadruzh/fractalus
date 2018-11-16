import * as wasm from "rustfractals";

let result = wasm.wasmdraw(100, 100, 100);

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
    console.log(tmpArr);

    return resultArr;
};

const var_1 = (result) => {

    var res = [];

    for (let i = 0; i <= result.length; i += 5) {
        res.push(result.slice(i, i + 3));
    }
}

console.log("var1", var_2(wasm.wasmdraw(100, 100, 100)))
console.log("var2", var_2(wasm.wasmdraw(100, 100, 100)))

let fractal_array = var_2(wasm.wasmdraw(100, 100, 100));

var canvas = document.querySelector('.fractal');
var width = canvas.width = window.innerWidth;
var height = canvas.height = window.innerHeight;
var ctx = canvas.getContext('2d');

ctx.fillStyle = 'rgb(255, 0, 0)';
ctx.fillRect(50, 50, 100, 150);


fractal_array.forEach(function (el) {
    console.log(el)
})
