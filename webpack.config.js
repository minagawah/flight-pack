const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const Dotenv = require('dotenv-webpack');
const TerserPlugin = require('terser-webpack-plugin');

const dist = path.resolve(__dirname, 'dist');

const ENV_NAME = process.env.NODE_ENV;
const PROD = ENV_NAME === 'production';

module.exports = {
  mode: 'production',
  entry: {
    index: './js/index.js',
  },
  output: {
    path: dist,
    filename: '[name].[fullhash].js',
  },
  resolve: {
    extensions: ['.js'],
  },
  experiments: {
    asyncWebAssembly: true,
  },
  performance: {
    hints: false,
    // maxEntrypointSize: 512000,
    maxAssetSize: 512000, // 500 bytes
    // maxAssetSize: 3145728, // 3 MB
    // maxAssetSize: 4194304, // 4 MB
    // maxAssetSize: 5242880, // 5 MB
    // maxAssetSize: 6291456, // 6 MB
    // maxAssetSize: 7340032, // 7 MB
  },
  optimization: {
    ...(PROD && {
      minimize: true,
      minimizer: [
        new TerserPlugin({
          terserOptions: {
            mangle: true,
          },
        }),
      ],
    }),
  },
  devtool: PROD ? 'cheap-source-map' : 'inline-source-map',
  devServer: {
    // contentBase: dist,
    static: {
      directory: dist,
    },
    port: 8080,
    devMiddleware: {
      writeToDisk: true,
    },
  },
  module: {
    rules: [
      {
        test: /\.m?js?$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
        },
      },
      {
        test: /\.css$/,
        use: ['style-loader', 'css-loader', 'postcss-loader'],
      },
    ],
  },
  plugins: [
    new Dotenv(),
    new CleanWebpackPlugin(),
    new HtmlWebpackPlugin({
      template: './js/index.html',
      filename: 'index.html',
      minify: { collapseWhitespace: false },
    }),
    new WasmPackPlugin({
      crateDirectory: __dirname,
      forceMode: PROD ? 'production' : 'development',
    }),
  ],
};
