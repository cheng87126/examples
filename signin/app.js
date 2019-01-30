const Koa = require('koa')
const moment = require('moment')
const schedule = require('node-schedule')

const app = new Koa()

app.use(async ctx => {
	ctx.body = moment().day()
})

function sign(){
	let day = moment().add(1, 'day').day(),
		date = {
			hour: day===0||day===6?21:12,
			minute: 30,
			second: 0,
			dayOfWeek: day
		}

	let s = schedule.scheduleJob(date, function(){
		console.log(new Date())
		s.cancel()
		sign()
	})
}
sign()

app.listen(3000)