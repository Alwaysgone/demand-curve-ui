# Demand Curve UI

[![demand-curve-ui](https://github.com/Alwaysgone/demand-curve-ui/actions/workflows/pipeline.yml/badge.svg?branch=master)](https://github.com/Alwaysgone/demand-curve-ui/actions/workflows/pipeline.yml) [![Demo](https://img.shields.io/badge/Demo-blue)](https://alwaysgone.github.io/demand-curve-ui/www/)

*Running without npm:*

Run `wasm-pack build --target web` in the project root to build the web assembly and start a web server in the project root e.g. `basic-http-server`. The app is now available on the default port of your web server under the `www` context path, so in case of using `basic-http-server` the app is available at `http://127.0.0.1:4000/www`.

*Running with npm:*

Run `wasm-pack build` in the project root to build the web assembly and run `npm install` in the `www` directory. When this is done it can be started using `npm run start` in the  `www` directory.
Changes to the web assembly are automatically available if `wasm-pack build` is executed in the project root with the previously started server.

## Example

![Example Demand Curve](/img/example_demand_curve.png)