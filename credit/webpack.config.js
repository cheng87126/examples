const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const CleanWebpackPlugin = require('clean-webpack-plugin')

let config = {
	entry: './app/public/src/index.js',
	output: {
		filename: 'app.js',
		path: path.resolve(__dirname, '/app/public/dist')
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
		contentBase: './dist'
	},
	plugins: [
		new CleanWebpackPlugin(['dist']),
		new HtmlWebpackPlugin({
			filename: './index.html',
			template: './app/view/index.html',
			inject: true
		})
	]
}

module.exports = (env, argv)=>{
	if (argv.mode === 'development') {
		config.devtool = 'source-map'
	}
	if (argv.mode === 'production') {

	}
	
	return config
}