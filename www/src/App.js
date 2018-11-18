import React, { Component } from 'react';
import logo from './logo.svg';
import './App.css';
import * as wasm from "rustfractals";


class App extends Component {

  constructor(props) {
    super(props);

    this.getPixelsArray = this.getPixelsArray.bind(this);
    this.componentToHex = this.componentToHex.bind(this);
    this.rgbToHex = this.rgbToHex.bind(this);

  }

  getPixelsArray = (result) => {

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


  componentToHex = (x) => {
    var hex = x.toString(16);
    return hex.length == 1 ? "0" + hex : hex;
  }

  rgbToHex = (r, g, b) => {
    return "#" + this.componentToHex(r) + this.componentToHex(g) + this.componentToHex(b);
  }


  render() {

    let fractal_array = this.getPixelsArray(wasm.wasmdraw(500, 500, 500));
    var canvas = document.querySelector('.fractal');
    var ctx = canvas.getContext('2d');

    fractal_array.forEach(function (el) {
      let color = this.rgbToHex(el[2], el[3], el[4]);
      ctx.fillStyle = color;
      ctx.fillRect(el[0], el[1], 1, 1);
    })

    return (
      <div className="App container-fluid">
        <canvas class="fractal" width="500" height="500"></canvas>
      </div>
    );
  }
}

export default App;
