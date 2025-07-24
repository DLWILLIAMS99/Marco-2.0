const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const WorkboxPlugin = require('workbox-webpack-plugin');

module.exports = (env, argv) => {
  const isProduction = argv.mode === 'production';
  
  return {
    entry: './src/web/index.ts',
    output: {
      path: path.resolve(__dirname, 'dist'),
      filename: isProduction ? '[name].[contenthash].js' : '[name].js',
      chunkFilename: isProduction ? '[name].[contenthash].chunk.js' : '[name].chunk.js',
      clean: true,
    },
    
    module: {
      rules: [
        {
          test: /\.tsx?$/,
          use: 'ts-loader',
          exclude: /node_modules/,
        },
        {
          test: /\.css$/,
          use: [
            isProduction ? MiniCssExtractPlugin.loader : 'style-loader',
            'css-loader',
          ],
        },
        {
          test: /\.(png|jpe?g|gif|svg|woff2?|ttf|eot)$/,
          type: 'asset/resource',
          generator: {
            filename: 'assets/[name].[contenthash][ext]',
          },
        },
        {
          test: /\.wasm$/,
          type: 'asset/resource',
          generator: {
            filename: 'wasm/[name].[contenthash][ext]',
          },
        },
      ],
    },
    
    resolve: {
      extensions: ['.tsx', '.ts', '.js', '.wasm'],
      alias: {
        '@': path.resolve(__dirname, 'src/web'),
      },
    },
    
    plugins: [
      new CleanWebpackPlugin(),
      
      new HtmlWebpackPlugin({
        template: './src/web/index.html',
        title: 'Marco 2.0 - Visual IDE',
        meta: {
          viewport: 'width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no, viewport-fit=cover',
          'theme-color': '#2563eb',
          'apple-mobile-web-app-capable': 'yes',
          'apple-mobile-web-app-status-bar-style': 'black-translucent',
          'apple-mobile-web-app-title': 'Marco 2.0',
        },
        minify: isProduction,
      }),
      
      new CopyWebpackPlugin({
        patterns: [
          {
            from: path.resolve(__dirname, 'pkg'),
            to: 'pkg',
            globOptions: {
              ignore: ['**/package.json', '**/README.md'],
            },
          },
          {
            from: path.resolve(__dirname, 'src/web/assets'),
            to: 'assets',
            noErrorOnMissing: true,
          },
          {
            from: path.resolve(__dirname, 'src/web/manifest.json'),
            to: 'manifest.json',
            noErrorOnMissing: true,
          },
        ],
      }),
      
      ...(isProduction ? [
        new MiniCssExtractPlugin({
          filename: '[name].[contenthash].css',
          chunkFilename: '[id].[contenthash].css',
        }),
        
        new WorkboxPlugin.GenerateSW({
          clientsClaim: true,
          skipWaiting: true,
          runtimeCaching: [
            {
              urlPattern: /\.(?:png|jpg|jpeg|svg|gif|webp)$/,
              handler: 'CacheFirst',
              options: {
                cacheName: 'images',
                expiration: {
                  maxEntries: 100,
                  maxAgeSeconds: 30 * 24 * 60 * 60, // 30 days
                },
              },
            },
            {
              urlPattern: /\.(?:wasm)$/,
              handler: 'CacheFirst',
              options: {
                cacheName: 'wasm',
                expiration: {
                  maxEntries: 10,
                  maxAgeSeconds: 7 * 24 * 60 * 60, // 7 days
                },
              },
            },
          ],
        }),
      ] : []),
    ],
    
    optimization: {
      splitChunks: {
        chunks: 'all',
        cacheGroups: {
          vendor: {
            test: /[\\/]node_modules[\\/]/,
            name: 'vendors',
            chunks: 'all',
          },
          wasm: {
            test: /\.wasm$/,
            name: 'wasm',
            chunks: 'all',
          },
        },
      },
    },
    
    devServer: {
      static: {
        directory: path.join(__dirname, 'dist'),
      },
      compress: true,
      port: 8080,
      hot: true,
      historyApiFallback: true,
      headers: {
        'Cross-Origin-Embedder-Policy': 'require-corp',
        'Cross-Origin-Opener-Policy': 'same-origin',
      },
    },
    
    experiments: {
      asyncWebAssembly: true,
      topLevelAwait: true,
    },
    
    performance: {
      maxAssetSize: isProduction ? 5 * 1024 * 1024 : 10 * 1024 * 1024, // 5MB prod, 10MB dev
      maxEntrypointSize: isProduction ? 5 * 1024 * 1024 : 10 * 1024 * 1024,
    },
  };
};
