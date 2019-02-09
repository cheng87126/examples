const Koa = require('koa')
const moment = require('moment')
const schedule = require('node-schedule')
const fs = require('fs')

const app = new Koa()

app.use(async ctx => {
	ctx.body = moment().day()
})

function sign(){
	let today = moment().day(),
		day = moment().add(1, 'day').day(),
		date = {
			hour: day===0||day===6?21:12,
			minute: 30,
			second: 0,
			dayOfWeek: day
		}

	let s = schedule.scheduleJob(date, ()=>{
		console.log(new Date())

		fs.appendFile('test', moment().format('YYYY-MM-DD hh:mm:ss a')+ date.dayOfWeek +'\n',  (err) => {
			if (err) {
				return console.error(err)
			}
		})

		s.cancel()
		sign()
	})
}
sign()

app.listen(3000)