
export async function fetcher(args:any){
    let { url, method='GET', data } = args
    let opt
    switch (method) {
        case 'GET':
            const params = new URLSearchParams(data)
            url += `?${params.toString()}`
            break
        case 'POST':
            opt = {
                method,
                headers: {
                  "Content-Type": "application/json"
                },
                body: JSON.stringify(data)
            }
            break
    }
    const res = await fetch(url,opt)
    return await res.json()
  }