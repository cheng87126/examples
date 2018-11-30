const path = require('path')
const webpack = require('webpack')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const CleanWebpackPlugin = require('clean-webpack-plugin')
// const BundleAnalyzerPlugin = require('webpack-bundle-analyzer').BundleAnalyzerPlugin

let config = {
	entry: './app/public/src/index.js',
	output: {
		filename: 'app.js',
		chunkFilename: '[name].js',
		path: path.join(__dirname, '/app/public/dist'),
		pathinfo: false
	},
	module:{
		rules:[
			{
				test: /\.(js|jsx)$/,
				exclude: /node_modules/,
				loader: "babel-loader",
			},
			{
				test: /\.(png|jpg|gif|svg)$/,
				use: [
					{
						loader: 'url-loader',
						options: {
							limit: 8192
						}
					}
				]
			}
		]
	},
	devServer: {
		contentBase: './dist',
		historyApiFallback: true
	},
	plugins: [
		// new BundleAnalyzerPlugin(),
		new CleanWebpackPlugin(['./app/public/dist']),
		new HtmlWebpackPlugin({
			filename: './index.html',
			template: './app/view/index.html',
			inject: true
		})
	],
	watchOptions: {
		ignored: /node_modules/
	}
}

module.exports = (env, argv)=>{
	if (argv.mode === 'development') {
		config.devtool = 'source-map'
	}
	if (argv.mode === 'production') {
		config.output = {
			...config.output,
			filename: '[name].[hash].js',
			chunkFilename: '[name].[hash].js'
		}
		// config.plugins.push(
		// 	new webpack.DefinePlugin({
		// 		'process.env.NODE_ENV': JSON.stringify('production')
		// 	})
		// )
		config.optimization = {
			splitChunks:{
				cacheGroups:{
					vendor:{
						chunks: 'initial',
						test: /react/,
						name: 'vendor',
						enforce: true
					}
				}
			}
		}
	}
	
	return config
}