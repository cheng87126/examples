const path = require('path')
const VueLoaderPlugin = require('vue-loader/lib/plugin')

module.exports = {
	entry: './entry-server.js',
	output: {
		filename: 'bundle-server.js',
		path: path.resolve(__dirname, 'dist')
	},
	module:{
		rules:[
			{
				test: /\.vue$/,
				use:[
					'vue-loader'
				]
			},
			{
				test: /\.js$/,
				loader: 'babel-loader'
			},
			{
				test: /\.css$/,
				use: [
					'vue-style-loader',
					'css-loader'
				]
			}
		]
	},
	plugins: [
		new VueLoaderPlugin()
	]
}