import { makeAutoObservable, runInAction } from 'mobx'

export default class RootStore {
    userStore
    todoStore
    constructor() {
        console.log('RootStore')
        this.userStore = new UserStore(this)
        this.todoStore = new TodoStore(this)
    }
}

class UserStore {
    rootStore
    constructor(rootStore:RootStore) {
        console.log('UserStore')
        this.rootStore = rootStore
    }

    getTodos(user:string) {
        console.log('user',user)
        // 通过 root store 来访问 todoStore
        return this.rootStore.todoStore.todos.filter(todo => todo.author === user)
    }
}

class TodoStore {
    todos:{author:string}[] = []
    rootStore

    constructor(rootStore:RootStore) {
        console.log('TodoStore')
        makeAutoObservable(this, { rootStore: false })
        this.rootStore = rootStore
        // this.fetchTodos()
    }

    async fetchTodos(){
        await fetch('http://localhost:3000/getdata')
        runInAction(()=>{
            this.todos = [{author:'aaa'}]
        })
    }
}