import { fetcher } from '@/utils'

export function getFundName(code:string){
  return fetcher({url:'/api/getFundName',data:{code}})
}
export function addOrUpdateFund(data:Record<string,any>){
  return fetcher({url:data.id?'api/updateFund':'api/addFund',method:'POST',data})
}