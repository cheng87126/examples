import { map } from 'rxjs/operators'
import { fromEvent, merge } from 'rxjs'

const $ = (el) => document.querySelector(el)

let input1 = $('#input1'),
	input2 = $('#input2')

const clicks = fromEvent(input1,'click')

clicks.subscribe(x => console.log(x))