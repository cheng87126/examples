import { Form, Input, Button } from 'antd'
import { addOrUpdateFund } from '@/api/favorites'

type FieldType = {
  content:string
  remark:string
}

interface Props{
  mutate:()=>void
}

export default function AddorEdit({mutate}:Props){
  const [form] = Form.useForm()

  const onFinish = async function(val:FieldType){
    await addOrUpdateFund(val)

    mutate()
  }

  return (
    <Form
      form={form}
      layout="inline"
      initialValues={{}}
      onFinish={onFinish}
      autoComplete="off"
      className="my-4"
    >
      <Form.Item<FieldType>
        label="内容"
        name="content"
        rules={[{ required: true, message: '请输入内容' }]}
      >
        <Input addonBefore="https://" placeholder="请输入内容" />
      </Form.Item>
      <Form.Item<FieldType>
        label="备注"
        name="remark"
        rules={[{ required: true, message: '请输入备注' }]}
      >
        <Input placeholder="请输入备注" />
      </Form.Item>
      <Form.Item noStyle>
        <Button type="primary" htmlType="submit">添加</Button>
      </Form.Item>
    </Form>
  )
}