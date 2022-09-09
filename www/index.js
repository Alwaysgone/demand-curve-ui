//import * as wasm from "demand-curve-ui";

//wasm.greet("Michael");


class Chart {}

const canvas = document.getElementById("canvas");
const status = document.getElementById("status");

let chart = null;


/** Main entry point */
export function main() {
    setupCanvas();
}

/** This function is used in `bootstrap.js` to setup imports. */
export function setup(WasmChart) {
    Chart = WasmChart;
}

/** Setup canvas to properly handle high DPI and redraw current plot. */
function setupCanvas() {
	const dpr = window.devicePixelRatio || 1.0;
    const aspectRatio = canvas.width / canvas.height;
    const size = canvas.parentNode.offsetWidth * 0.8;
    canvas.style.width = size + "px";
    canvas.style.height = size / aspectRatio + "px";
    canvas.width = size;
    canvas.height = size / aspectRatio;
    updatePlot();
}

/** Redraw currently selected plot. */
function updatePlot() {
    status.innerText = `Rendering chart...`;
    chart = null;
    const start = performance.now();
	//chart = Chart.power("canvas", 5)
	chart = Chart.demandCurve("canvas");
    const end = performance.now();
    status.innerText = `Rendered chart in ${Math.ceil(end - start)}ms`;
}



