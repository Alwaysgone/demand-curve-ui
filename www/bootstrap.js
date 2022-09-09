// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.
//import("./index.js")
//  .catch(e => console.error("Error importing `index.js`:", e));

  init();

async function init() {
    if (typeof process == "object") {
        // We run in the npm/webpack environment.
        const [{Chart}, {main, setup}] = await Promise.all([
            import("demand-curve-ui"),
            import("./index.js"),
        ]);
        setup(Chart);
        main();
    } else {
        const [{Chart, default: init}, {main, setup}] = await Promise.all([
            import("../pkg/demand_curve_ui.js"),
            import("./index.js"),
        ]);
        await init();
        setup(Chart);
        main();
    }
}
  