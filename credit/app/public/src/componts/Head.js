import React from 'react'
import { connect } from 'react-redux'

class Head extends React.Component{
	constructor(props){
		super(props)
	}
	render(){
		return (
			<header className="head">
				<div>{this.props.title}</div>
				<style jsx>{`
					.head{
						font-size:20px;
						div{
							color:#999;
						}
					}
				`}</style>
			</header>
		)
	}
}

const mapStateToProps = state =>{
	return {title:state.headTxt}
}

export default connect(mapStateToProps)(Head)