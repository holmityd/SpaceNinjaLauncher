/* eslint-disable */
const resolve = require("path").resolve;
const webpack = require("webpack");
const TsconfigPathsPlugin = require("tsconfig-paths-webpack-plugin");
const CopyPlugin = require("copy-webpack-plugin");
const Dotenv = require("dotenv-webpack");
const config = require("./SpaceNinjaServer/config.json");
const TerserPlugin = require("terser-webpack-plugin");

module.exports = {
  target: "node",
  mode: "production",
  node: {
    __dirname: false,
    __filename: false,
  },
  plugins: [
    new Dotenv({ path: "./SpaceNinjaServer/.env" }),
    new webpack.ContextReplacementPlugin(
      /express\/lib/,
      resolve(__dirname, "../node_modules"),
      {
        ejs: "ejs",
      }
    ),
    new CopyPlugin({
      patterns: [
        { from: "./SpaceNinjaServer/static/certs/", to: "certs/" },
        `./SpaceNinjaServer/static/data/H.Cache_${config.version}.bin`,
      ],
    }),
  ],
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      {
        test: /cache\.ts$/,
        loader: "string-replace-loader",
        options: {
          search: "../../static/data/",
          replace: "./",
          flags: "g",
        },
      },
      {
        test: /index\.ts$/,
        loader: "string-replace-loader",
        options: {
          search: "../static/certs/",
          replace: "certs/",
          flags: "g",
        },
      },
    ],
  },
  resolve: {
    plugins: [
      new TsconfigPathsPlugin({
        configFile: "./SpaceNinjaServer/tsconfig.json",
      }),
    ],
    fallback: {
      "mongodb-client-encryption": false,
      aws4: false,
      snappy: false,
      "@aws-sdk/credential-providers": false,
      "@mongodb-js/zstd": false,
      kerberos: false,
    },
    extensions: [".tsx", ".ts", ".js"],
  },
  optimization: {
    minimize: true,
    minimizer: [
      new TerserPlugin({
        extractComments: false,
      }),
    ],
  },
  entry: ["./SpaceNinjaServer/src/index.ts"],
  output: {
    filename: "index.js",
    path: resolve(__dirname, "build"),
  },
};
