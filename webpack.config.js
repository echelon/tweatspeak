var webpack = require('webpack');

module.exports = {
  entry: {
    app: './www/script/app.ts',
  },
  output: {
    path: './www/build',
    filename: '[name].built.js'
  },
  resolve: {
    extensions: ['', '.ts', '.js']
  },
  module: {
    loaders: [
      { test: /\.ts$/, loader: 'ts-loader' },
    ]
  },
  plugins: [
    new webpack.ProvidePlugin({
      $: "jquery",
      buzz: "buzz",
      "window.buzz": "buzz",
    }),
  ],
}
