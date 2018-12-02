import React, {Component} from 'react';
import logo from './logo.svg';
import './App.css';
//import { wasmdraw } from 'rustfractals';
// import * as wasm from "rustfractals";
//import("../node_modules/rustfractals");


class App extends Component {

    constructor(props) {
        super(props);

        this.state = {
            fractalArray: [],
            windowHeight: 0,
            windowWidth: 0,
        }

        this.getPixelsArray = this.getPixelsArray.bind(this);
        this.componentToHex = this.componentToHex.bind(this);
        this.rgbToHex = this.rgbToHex.bind(this);
        this.updateCanvas = this.updateCanvas.bind(this);
        this.setWindowSize = this.setWindowSize.bind(this);
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

    componentDidMount() {

        import('../node_modules/rustfractals')
            .then(wasm => {
                return wasm.wasmdraw(window.innerHeight, window.innerHeight, 100);
            })
            .catch(e => console.log("error: ", e))
            .then(result => {
                this.setState({
                    fractalArray: this.getPixelsArray(result),
                    windowHeight: window.innerHeight,
                    windowWidth: window.innerHeight
                });
                this.updateCanvas();
            });
    }

    updateCanvas() {
        // e.preventDefault();
        const ctx = this.refs.canvas.getContext('2d');

        this.state.fractalArray.forEach(function (el) {
            // console.log("el: ", el)
            let color = "#"
                + (el[2].toString(16).length == 1 ? "0" + el[2].toString(16) : el[2].toString(16))
                + (el[3].toString(16).length == 1 ? "0" + el[3].toString(16) : el[3].toString(16))
                + (el[4].toString(16).length == 1 ? "0" + el[4].toString(16) : el[4].toString(16)); //this.rgbToHex(el[2], el[3], el[4]);
            ctx.fillStyle = color;
            ctx.fillRect(el[0], el[1], 1, 1);
        })
    }

    setWindowSize() {
        this.setState({
            windowHeight: document.innerHeight,
            windowWidth: document.innerWidth
        })
    }

    render() {

        // var canvas = document.querySelector('.fractal');
        // var ctx = canvas.getContext('2d');

        // this.state.fractalArray.forEach(function (el) {
        //   let color = this.rgbToHex(el[2], el[3], el[4]);
        //   ctx.fillStyle = color;
        //   ctx.fillRect(el[0], el[1], 1, 1);
        // })

        return (
            <div className="App container-fluid">
                <canvas className="fractal" ref="canvas" id="fractal" width={this.state.windowWidth} height={this.state.windowHeight}></canvas>
            </div>
        );
    }
}

export default App;
