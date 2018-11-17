import { combineReducers } from 'redux'

function counter(num = 0, action) {
	switch (action.type) {
		case 'INCREMENT':
			return num + 1
		case 'DECREMENT':
			return num - 1
		default:
			return num
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
	counter,
	headTxt	
})

export default appState