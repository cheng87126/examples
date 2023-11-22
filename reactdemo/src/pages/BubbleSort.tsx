import { flushSync } from 'react-dom'
import { useEffect, useState } from 'react'

export default function BubbleSort(){
  const [arr, setArr] = useState<number[]>([])

  useEffect(()=>{
    const ret:number[][] = []
    const arr = [20,23,11,21,22,40]
    for(let i=0;i<arr.length-1;i++){
      for(let j=i+1;j<arr.length;j++){
        if(arr[j]<arr[i]){
          const tmp = arr[i]
          arr[i] = arr[j]
          arr[j] = tmp
        }
        ret.push([...arr])
      }
    }
    let i = 0
    const timer = setInterval(()=>{
      if(i===ret.length){
        clearInterval(timer)
        return
      }
      flushSync(()=>{
        setArr(_=>{
          return [...ret[i]]
        })
      })
      
      i++
    },1500)

    return ()=>{
      clearInterval(timer)
    }
  },[])

  return (
    <div>{arr.join()}</div>
  )
}