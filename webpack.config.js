// const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./decay.js",
  output: {
    path: path.resolve(__dirname, "src/wasm-decay-pkg"),
    filename: "decay.js",
  },
  mode: "production"
};
