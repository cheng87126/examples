import React from 'react'
import { connect } from 'react-redux'

class Head extends React.Component{
	constructor(props){
		super(props)
	}
	render(){
		return (
			<header className="head">
				{this.props.title}
			</header>
		)
	}
}

const mapStateToProps = state =>{
	return {title:state.headTxt}
}

export default connect(mapStateToProps)(Head)