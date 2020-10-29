// This webpack file is only here for karmatic. Once karmatic's
// master branch is released (with native rollup functionality)
// this file can be removed (and webpack removed as well)
module.exports = {
  mode: "development",
  devtool: "inline-source-map",
  resolve: {
    extensions: [".ts", ".tsx", ".js"],
  },
  module: {
    rules: [{ test: /\.tsx?$/, loader: "ts-loader" }],
  },
};
