import useSWR from 'swr'
import { List, Typography } from 'antd'
import AddorEdit from './AddorEdit'

interface ListItem{
  content:string
  remark:string
}
export default function Favorites(){
  const { data, mutate } = useSWR<ListItem[]>({url:'/api/getUrls'})

  return (
    <>
      <AddorEdit mutate={mutate} />
      <List
        header={null}
        footer={null}
        bordered
        dataSource={data}
        renderItem={(item) => (
          <List.Item>
            <Typography.Link href={item.content} target="_blank">{item.content}</Typography.Link>
            <Typography.Text>{item.remark}</Typography.Text>
          </List.Item>
        )}
      />
    </>
  )
}