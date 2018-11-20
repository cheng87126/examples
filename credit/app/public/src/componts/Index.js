import React from 'react'

import { exposure } from '../api'

class Index extends React.Component{
	constructor(props){
		super(props)
		this.state = {
			list:[]
		}
	}
	render(){
		let { list } = this.state 
		return (
			<ul>{
				list.map((i,idx)=>{
					return (
						<li key={idx}>
							<div>{i.date}</div>
							{i.content}
						</li>
					)
				})}
			</ul>
		)
	}
	componentDidMount(){
		exposure.getList()
			.then(res=>{
				console.log(res.data)
				this.setState({
					list:res.data
				})
			})
	}
}

export default Index