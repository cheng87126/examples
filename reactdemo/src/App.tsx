import React, { useState, useEffect } from 'react';
import {
  createBrowserRouter,
  RouterProvider
} from 'react-router-dom'
import { of, map, interval, take, takeUntil, concatAll, fromEvent } from 'rxjs'
import dayjs from 'dayjs'
import isBetween from 'dayjs/plugin/isBetween'
import RootStore from './store';
import Todos from './Todos'
import Calendar from './Calendar'
import logo from './logo.svg';
import { createEditor } from 'slate'
import { Slate, Editable, withReact } from 'slate-react'
import './App.css';
dayjs.extend(isBetween)
function overlap(p:{s:string;e:string},p1:{s:string;e:string}){
  return dayjs(p.s).isBetween(p1.s, p1.e, 'minute', '[]') || dayjs(p.s).isBetween(p1.s, p1.e, 'minute', '[]') || dayjs(p1.s).isBetween(p.s, p.e, 'minute', '[]')
}
const dateArr = [
  {s:'2023-03-18 06:00',e:'2023-03-18 08:30',n:0},
  {s:'2023-03-18 08:00',e:'2023-03-18 10:30',n:0},
  {s:'2023-03-18 05:00',e:'2023-03-18 18:30',n:0},
  {s:'2023-03-18 14:00',e:'2023-03-18 18:00',n:0},
  {s:'2023-03-19 10:00',e:'2023-03-19 12:00',n:0},
  {s:'2023-03-19 06:00',e:'2023-03-19 10:30',n:0}
]
for(let i=0;i<dateArr.length;i++){
  for(let j=i+1;j<dateArr.length;j++){
    if(overlap(dateArr[i],dateArr[j])){
      dateArr[i].n = (dateArr[i].n||0)+1
      dateArr[j].n = (dateArr[j].n||0)+1
    }
  }
}
console.log('dateArr',dateArr)
const rootStore = new RootStore()
const initialValue = [
  {
    type: 'paragraph',
    children: [{ text: 'A line of text in a paragraph.' }],
  }
]
function AppComponent() {
  const [editor] = useState(() => withReact(createEditor()))
  console.log('editor',editor)
  useEffect(()=>{
    const obs1 = interval(1000).pipe(take(5))
    const obs2 = interval(500).pipe(take(2))
    const obs3 = interval(2000).pipe(take(1))
    /*
    of(obs1, obs2, obs3)
      .subscribe((v) => {
        v.subscribe((vv)=>console.log(`vv: ${vv}`))
        console.log(`value: ${v}`)
      })
    */
    of(obs1, obs2, obs3).pipe(
      concatAll()
    ).subscribe(c=>console.log(`c: ${c}`))

    const dragDOM = document.getElementById('drag')!;
    const body = document.body;
    const mouseDown = fromEvent(dragDOM, 'mousedown');
    const mouseUp = fromEvent(body, 'mouseup');
    const mouseMove = fromEvent(body, 'mousemove');
    mouseDown
      .pipe(
        map(event => mouseMove.pipe(takeUntil(mouseUp))),
        concatAll(),
        // React.MouseEvent<HTMLDivElement, MouseEvent>
        map((event:any) => ({ x: event.clientX, y: event.clientY }))
      )
      .subscribe(pos => {
        dragDOM.style.left = pos.x + 'px';
        dragDOM.style.top = pos.y + 'px';
      })
  },[])

  return (
    <div className="App">
      <Slate editor={editor} initialValue={initialValue}>
        <Editable />
      </Slate>
      <Calendar />
      <ul>
        {Array(10).fill(0).map((item,idx)=>(
          <li
            key={idx}
            draggable
            style={{marginBottom:'4px'}}
            onDragStart={e=>{
              e.dataTransfer.dropEffect = 'move'
              e.dataTransfer.setData('text/plain', 'Text to drag')
              console.log(e,idx)
            }}
            onDragOver={e=>{e.preventDefault()}}
            onDrop={e=>{
              e.preventDefault()
              console.log(e,idx)
              const data = e.dataTransfer.getData('text/plain')
              // e.target.textContent = data
              console.log('data',data)
            }}
          >
            {item}---{idx}
          </li>
        ))}
      </ul>
      <table>
        <thead>
          <tr>
            <th>thead</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>
              {Array(100).fill(0).map((item,idx)=>(
                <div key={idx}>tbody</div>
              ))}
            </td>
          </tr>
        </tbody>
        <tfoot>
          <tr><td>tfoot</td></tr>
        </tfoot>
      </table>
      <Todos store={rootStore} />
      <div id="drag" style={{width:'100px',height:'100px',border:'1px solid #000',position:'absolute'}} />
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

const router = createBrowserRouter([
  {
    path: '/',
    element: <AppComponent />
  },
  
])
function App(){

  return (
    <RouterProvider router={router} />
  )
}

export default App;
