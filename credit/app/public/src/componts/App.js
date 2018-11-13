import React from 'react'
import Head from './Head'
import Footer from './Footer'

export default class extends React.Component{
	constructor(props){
		super(props)
	}
	render(){
		return (
			<>
				<Head />
				<main className="main">
					main
				</main>
				<Footer />
			</>
		)
	}
}