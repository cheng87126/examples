import { observer } from "mobx-react-lite"
import RootStore from './store'

const Todos = function({store}:{store:RootStore}){
    const data = store.userStore.getTodos('aaa')
    console.log('Todos')
    return (
        <div>
            <button onClick={()=>store.todoStore.fetchTodos()}>get</button>
            <ul>
                {data.map(item=><li key={item.author}>{item.author}</li>)}
            </ul>
        </div>
    )
}
export default observer(Todos)