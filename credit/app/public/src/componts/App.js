import React from 'react'
import {
	BrowserRouter as Router,
	Switch,
	Route,
	Link
} from 'react-router-dom'

import Head from './Head'
import Footer from './Footer'
import Index from './Index'

const My = () => (
	<div>
		My
	</div>
)
const Detail = () => (
	<div>detail</div>
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
							<Switch>
								<Route exact path="/" component={Index}/>
								<Route path="/my" component={My}/>
								<Route path="/detail" component={Detail}/>
							</Switch>
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