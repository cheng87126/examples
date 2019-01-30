import { map } from 'rxjs/operators'
import { fromEvent, merge, interval } from 'rxjs'

import spritejs from 'spritejs'

const $ = (el) => document.querySelector(el)

let input1 = $('#input1'),
	input2 = $('#input2')

const clicks = fromEvent(input1,'click')

clicks.subscribe(x => console.log(x))

// interval(1000).subscribe(x => console.log(x))

const { Scene, Sprite } = spritejs
const scene = new Scene('#container', {
	viewport: ['auto', 'auto'],
	resolution: [800, 800],
})

const layer = scene.layer()

let boxConf = {
		size: [300, 50],
		pos: [100, 30],
		border: [2, '#f77'],
	},
	y = 30,
	boxs = Array(5).fill(1).map((item) => {
		let boxItem = {
			...boxConf,
			pos: [100, y]
		}
		y = y + 50 + 2
		return  new Sprite(boxItem)
	})

layer.append(...boxs)
console.log(boxs[4].xy[1])
let box = new Sprite({
	...boxConf,
	bgcolor:'#eb4e4e'
})
layer.append(box)
box.transition(3)
    .attr({
		pos: [100, boxs[4].xy[1]]
	})