
const weekTxt = ['一','二','三','四','五','六','日']
const Calendar = function(){

  return (
    <div>
      <div>
        {weekTxt.map(item=>(
          <span key={item}>{item}</span>
        ))}
      </div>
      
    </div>
  )
}
export default Calendar