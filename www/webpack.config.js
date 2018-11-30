const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./src/index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "main.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['./public/index.html'])
  ],
   module: {
     rules: [
       {
         test: /\.js$/,
         exclude: /node_modules/,
         use: {
           loader: 'babel-loader'
         }
       },
       {
        test: /\.css$/,
        use: [ 'style-loader', 'css-loader' ]
       },
           {
               test: /\.svg$/,
               loader: 'svg-inline-loader'
           }
     ]
   }
};
