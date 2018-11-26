import 'babel-polyfill'
import React from 'react'
import ReactDOM from 'react-dom'

import { createStore, applyMiddleware } from 'redux'
import createSagaMiddleware from 'redux-saga'
import { Provider } from 'react-redux'
import appState from './reducers'
import rootSaga from './saga'

import App from './componts/App'

const sagaMiddleware = createSagaMiddleware()
let store = createStore(appState,applyMiddleware(sagaMiddleware))
sagaMiddleware.run(rootSaga)

ReactDOM.render(
	<Provider store={store}>
		<App />
	</Provider>,
	document.getElementById('app')
)