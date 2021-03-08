const path = require('path');

const webpack = require('webpack');
const autoprefixer = require('autoprefixer');
const cssnano = require('cssnano');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const isDev = (mode) => mode === 'development';
const isProd = (mode) => mode === 'production';

module.exports = (_, argv) => ({
  entry: ['./client/index.ts', './client/index.scss'],
  output: {
    path: path.resolve(__dirname, './public'),
    filename: '[name].js',
  },
  mode: argv.mode,
  devtool: isDev(argv.mode) ? 'source-map' : 'cheap-source-map',
  module: {
    rules: [
      {
        test: /\.ts$/,
        exclude: /node_modules/,
        use: ['babel-loader', 'ts-loader'],
      },
      {
        test: /\.(sa|sc|c)ss$/,
        exclude: /node_modules/,
        use: [
          MiniCssExtractPlugin.loader,
          {
            loader: 'css-loader',
            options: { importLoaders: 2 },
          },
          {
            loader: 'postcss-loader',
            options: {
              postcssOptions: {
                ident: 'postcss',
                plugins: [autoprefixer()].concat(
                  isProd(argv.mode) ? [cssnano()] : []
                ),
              },
            },
          },
          'sass-loader',
        ],
      },
    ],
  },
  resolve: {
    alias: {
      '@magmsg/components': path.resolve(__dirname, './client/components'),
    },
    extensions: ['.ts', '.js', '.css'],
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: '[name].css',
      chunkFilename: '[id].css',
    }),
    new CleanWebpackPlugin({
      verbose: isDev(argv.mode),
      cleanOnceBeforeBuildPatterns: ['artifacts', '*.gz', '*.js', '*.css'],
    }),
    new webpack.EnvironmentPlugin({
      NODE_ENV: argv.mode,
    }),
  ],
  stats: {
    colors: true,
  },
});
