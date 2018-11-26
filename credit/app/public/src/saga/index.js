import { call, put, takeEvery } from 'redux-saga/effects'
import { exposure } from '../api/index'

function* getList(){
	try {
		const list = yield call(exposure.getList)
		yield put({type:'SET',playload:list.data})
	} catch (e) {
		console.log(e)
	}
}

function* rootSaga(){
	yield takeEvery('LIST_FETCH_REQUESTED', getList)
}

export default rootSaga