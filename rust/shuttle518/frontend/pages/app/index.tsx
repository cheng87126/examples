import { useState, useRef } from 'react'
import useSWR from 'swr'
import { Button, Table, Modal } from 'antd'
import AddorEdit from './AddorEdit'

export default function Home() {
  const addorEditRef = useRef<any>(null)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [editId, setEditId] = useState(0)

  const { data, isLoading } = useSWR({url:'/api/getFunds'})

  const showModal = () => {
    setIsModalOpen(true)
  }

  const handleOk = () => {
    addorEditRef.current?.submit()
  }

  const handleCancel = () => {
    setIsModalOpen(false)
  }

  const formatter = new Intl.NumberFormat()
  const onEdit = function(id:number){
    setEditId(id)
    showModal()
  }
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
      <div className="text-right my-2 mr-2.5"><Button type="primary" onClick={showModal}>添加</Button></div>
      <Table rowKey="id" loading={isLoading} pagination={false} dataSource={data} columns={columns} />
      <Modal title="添加基金" open={isModalOpen} onOk={handleOk} onCancel={handleCancel}>
        <AddorEdit ref={addorEditRef} editId={editId} cb={handleCancel} />
      </Modal>
    </>
  )
}
