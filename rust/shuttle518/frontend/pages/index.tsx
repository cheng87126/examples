import useSWR from 'swr'
import { Button, Table } from 'antd'

export default function Home() {
  const { data, isLoading } = useSWR({url:'/api/getFunds'})
  // const { data:urlsData } = useSWR({url:'/api/getUrls'})

  const formatter = new Intl.NumberFormat()
  const onEdit = function(id:number){}
  const columns = [
    {
      title: '名称',
      dataIndex: 'name',
    },
    {
      title: '金额',
      dataIndex: 'amount',
    },
    {
      title: '买入日期',
      dataIndex: 'buy_date',
    },
    {
      title: '计算日期',
      dataIndex: 'date',
    },
    {
      title: '总收益',
      dataIndex: 'total',
      render:(val:number)=>formatter.format(val)
    },
    {
      title: '万份收益',
      dataIndex: 'unit',
      render:(val:number)=>formatter.format(val)
    },
    {
      title: '年化收益',
      dataIndex: 'year',
      render:(val:number)=>formatter.format(val)
    },
    {
      title: '操作',
      dataIndex: 'opertion',
      width: 100,
      render:(_:any,record:any)=><Button type="primary" size="small" onClick={_=>onEdit(record.id)}>编辑</Button>
    }
  ]
  return (
    <>
      <Table rowKey="id" loading={isLoading} pagination={false} dataSource={data} columns={columns} />
    </>
  )
}
