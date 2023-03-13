// const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./decay.js",
  output: {
    path: path.resolve(__dirname, "pkg"),
    filename: "decay.js",
  },
  mode: "development"
};
