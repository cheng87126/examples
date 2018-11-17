import React from 'react'
import {
	BrowserRouter as Router,
	Route,
	Link
} from 'react-router-dom'

import Head from './Head'
import Footer from './Footer'


const Index = () => (
	<div>
		Index
	</div>
)
const My = () => (
	<div>
		My
	</div>
)

export default class extends React.Component{
	constructor(props){
		super(props)
	}
	render(){
		return (
			<>
				<Head />
				<Router>
					<div>
						<main className="main">
							<Route exact path="/" component={Index}/>
							<Route path="/my" component={My}/>
						</main>
						<ul>
							<li><Link to="/">head</Link></li>
							<li><Link to="/my">foot</Link></li>
						</ul>
					</div>
				</Router>
				<Footer />
			</>
		)
	}
}