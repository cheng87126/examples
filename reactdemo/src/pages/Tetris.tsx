import { useEffect, useRef, useState } from 'react'

const TETROMINO = ['I','J','L','O','S','T','Z']
export default function(){
  const currentLineRef = useRef(0)
  const [speed, setSpeed] = useState(1500)
  const [grid, setGrid] = useState<number[][]>(Array(20).fill([]).map(_=>Array(10).fill(0)))

  const up = function(){

  }
  const right = function(){
    let newVal = grid.map((row,i)=>{
      const idx = (row as any).findLastIndex((item:any)=>item===1)
      if(idx!==-1 && idx<row.length-1){
        row.unshift(0)
        row.length = 10
      }
      return row
    })
    setGrid(newVal)
  }
  const down = function(){
    const nextLine = currentLineRef.current+1
    let flag = true
    let newVal = grid.map((row,i)=>{
      const col:(0|1|2)[] = row.map(col=>col<2?0:2)
      // 
      if(i<=currentLineRef.current && i>=currentLineRef.current-3){
        col[4] = 1
      }
      // 
      if(currentLineRef.current===19){
        return col.map(col=>col===1?2:0)
      }
      return col
    })
    if(nextLine<19){
      newVal[currentLineRef.current].forEach((item,idx)=>{
        if(newVal[nextLine][idx] === 2 && item === 1){
          flag = false
        }
      })
      if(!flag){
        newVal = newVal.map((row,i)=>{
          return row.map(col=>col>0?2:0)
        })
        setGrid(newVal)
        currentLineRef.current = 0
        return
      }
    }
    setGrid(newVal)
    currentLineRef.current = nextLine>19?0:nextLine
  }
  const left = function(){

  }
  useEffect(()=>{
    const keydown = function(e:KeyboardEvent){
      switch(e.key){
        case 'ArrowUp':
          up()
          break
        case 'ArrowRight':
          right()
          break
        case 'ArrowDown':
          down()
          break
        case 'ArrowLeft':
          left()
          break
      }
    }
    document.addEventListener('keydown',keydown,false)
    return function(){
      document.removeEventListener('keydown',keydown,false)
    }
  },[down])

  return (
    <section>
      {grid.map((row,rowIdx)=>{
        return (
          <div key={rowIdx} data-row={rowIdx} style={{display:'flex',justifyContent: 'center'}}>
            {row.map((col,colIdx)=>{
              return (
                <div
                  key={colIdx}
                  data-col={`${rowIdx}-${colIdx}`}
                  style={{width:'40px',height:'40px',background:col?'#000':'#fff'}}
                >
                  <div style={{color:'#666'}}>{col}</div>
                  <span style={{fontSize:'12px'}}>{`${rowIdx}-${colIdx}`}</span>
                </div>
              )
            })}
          </div>
        )
      })}
    </section>
  )
}