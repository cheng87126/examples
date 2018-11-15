import React from 'react'
import { connect } from 'react-redux'

class Footer extends React.Component{
	constructor(props){
		super(props)
	}
	render(){
		return (
			<footer className="footer">
				<div onClick={this.setHead.bind(this,'查信用')}>查信用</div>				
				<div onClick={this.setHead.bind(this,'我')}>我</div>
			</footer>
		)
	}
	setHead(title){
		this.props.dispatch({
			type:'SETHEAD',
			title
		})
	}
}

export default connect()(Footer)