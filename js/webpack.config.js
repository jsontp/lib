const path = require('path');
const TerserPlugin = require('terser-webpack-plugin');

// TODO: let it work without window

module.exports = (env) => {
  let obj = [{
    entry: './src/index.ts',
    devtool: 'inline-source-map',
    module: {
      rules: [
        {
          test: /\.tsx?$/,
          use: 'ts-loader',
          exclude: /node_modules|tests|dist/,
        },
      ],
    },
    resolve: {
      extensions: ['.tsx', '.ts', '.js'],
    },
    output: {
      globalObject: 'this',
      library: {
        name: 'accutime',
        type: 'umd',
      },
      path: path.resolve(__dirname, 'dist'),
    },
    /*optimization: {
      minimize: true,
      minimizer: [
        new TerserPlugin({ parallel: true })
      ]
    }*/
  }, {
    entry: './src/client.ts',
    devtool: 'inline-source-map',
    module: {
      rules: [
        {
          test: /\.tsx?$/,
          use: 'ts-loader',
          exclude: /node_modules|tests|dist/,
        },
      ],
    },
    resolve: {
      extensions: ['.tsx', '.ts', '.js'],
    },
    output: {
      globalObject: 'this',
      library: {
        name: 'accutime',
        type: 'commonjs-module',
      },
      path: path.resolve(__dirname, 'dist'),
    },
    /*optimization: {
      minimize: true,
      minimizer: [
        new TerserPlugin({ parallel: true })
      ]
    }*/
  }];

  if (env.mode === 'production') {
    obj[0].mode = 'production';
    obj[1].mode = 'production';
  }
  else {
    obj[0].mode = 'development';
    obj[1].mode = 'development';
  }

  if (env.optimize === "true") {
    obj[0].optimization = {
      minimize: true,
      minimizer: [
        new TerserPlugin({ parallel: true })
      ]
    };
    obj[0].output.filename = 'jsontp.min.js';
    obj[1].optimization = {
      minimize: true,
      minimizer: [
        new TerserPlugin({ parallel: true })
      ]
    };
    obj[1].output.filename = 'jsontp.min.mjs';
  }
  else {
    obj[0].output.filename = 'jsontp.js';
    obj[1].output.filename = 'jsontp.mjs';
  }
  return obj;
};