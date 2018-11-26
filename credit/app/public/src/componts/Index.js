import React from 'react'
import { connect } from 'react-redux'
import {
	Link
} from 'react-router-dom'

import { exposure } from '../api'

class Index extends React.Component{
	constructor(props){
		super(props)
		this.state = {
			list:[]
		}
		console.log(props)
	}
	// static getDerivedStateFromProps(nextProps, prevState) {}
	render(){
		let { list } = this.state,
			{ match, exposureList } = this.props
		return (
			<ul>
				{exposureList.map((i,idx)=>{
					return (
						<li key={idx}>
							<div>{i.date}</div>
							<Link to={`${match.url}detail`}>{i.content}</Link>
						</li>
					)
				})}
			</ul>
		)
	}
	// getSnapshotBeforeUpdate(prevProps, prevState) {}
	componentDidMount(){
		let { exposureList,dispatch } = this.props
		dispatch({
			type:'LIST_FETCH_REQUESTED',
			playload:'1024'
		})
		/*
		if(!exposureList.length){
			exposure.getList()
				.then(res=>{
					console.log(res.data)
					this.setState({
						list:res.data
					})
					dispatch({
						type:'SET',
						playload:res.data
					})
				})
		}else{
			this.setState({
				list:exposureList
			})
		}
		*/
	}
}

const mapStateToProps = state =>{
	return {exposureList:state.exposureList}
}

export default connect(mapStateToProps)(Index)