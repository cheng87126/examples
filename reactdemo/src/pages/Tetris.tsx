import { useEffect, useRef, useState } from 'react'

const TETROMINO = ['I','J','L','O','S','T','Z']
export default function(){
  const currentLineRef = useRef(0)
  const [speed, setSpeed] = useState(1500)
  const [grid, setGrid] = useState(Array(20).fill([]).map(_=>Array(10).fill(0)))

  useEffect(()=>{
    const timer = setInterval(()=>{
      setGrid(val=>{
        const newVal = val.map((row,i)=>{
          const col:(0|1|2)[] = row.map(col=>col<2?0:2)
          if(i<=currentLineRef.current && i>=currentLineRef.current-3){
            col[4] = 1
          }
          return col
        })

        const nextLine = currentLineRef.current+1
        currentLineRef.current = nextLine>19?0:nextLine

        return newVal
      })
    },speed)

    return ()=>{
      clearInterval(timer)
    }
  },[speed])

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
                  style={{width:'30px',height:'30px',background:col?'#000':'#fff'}}
                >
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