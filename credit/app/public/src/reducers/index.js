import { combineReducers } from 'redux'

function exposureList(list = [], action) {
	switch (action.type) {
		case 'SET':
			list = action.playload
			return list
		default:
			return list
	}
}

function headTxt(txt = '信用平台',action){
	switch (action.type) {
		case 'SETHEAD':
			return action.title
		default:
			return txt
	}
}

const appState = combineReducers({
	exposureList,
	headTxt	
})

export default appState