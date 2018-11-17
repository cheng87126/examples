import axios from 'axios'

const instance = axios.create({
	baseURL: 'https://easy-mock.com/mock/5bf0126e216bd01fe9418909/credit/'
})

instance.interceptors.response.use((res) => {
	return res.data
})

const createAPI = (url, method, config) => {
	config = config || {}
	return instance({
		url,
		method,
		...config
	})
}

const exposure = {
	getList:config => createAPI('exposure/list', 'get', config)
}

export {
	exposure
}