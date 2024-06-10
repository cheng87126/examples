import { fetcher } from '@/utils'

export function addOrUpdateFund(data:Record<string,any>){
  return fetcher({url:data.id?'':'/api/addUrl',method:'POST',data})
}