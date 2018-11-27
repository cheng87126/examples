const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const CleanWebpackPlugin = require('clean-webpack-plugin')

let config = {
	entry: './src/index.js',
	output: {
		filename: 'index.js',
		path: path.resolve(__dirname, './dist')
	},
	module:{
		rules:[
			{
				test: /\.js$/,
				exclude: /node_modules/,
				loader: "babel-loader",
			}
		]
	},
	devServer: {
		contentBase: './dist',
		historyApiFallback: true
	},
	plugins: [
		new CleanWebpackPlugin(['dist']),
		new HtmlWebpackPlugin({
			filename: './index.html',
			template: './src/index.html',
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

	}
	
	return config
}