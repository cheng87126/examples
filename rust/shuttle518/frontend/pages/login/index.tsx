import { Form, Input, Button } from 'antd'
import { fetcher } from '@/utils'

export default function Login(){
    const [form] = Form.useForm()

    const onFinish = async function(val:Record<string,any>){
        const params = new URLSearchParams(val)
        const res = await fetch(
            '/api/login',
            {
                method:'POST',
                headers: {
                  'Content-Type': 'application/x-www-form-urlencoded'
                },
                body: params.toString()
            }
        )
        console.log(res)
    }

    return (
        <Form
          form={form}
          onFinish={onFinish}
          className="w-1/3 mx-auto"
        >
            <Form.Item name="user_name" label="用户名" rules={[{ required: true }]}>
              <Input />
            </Form.Item>
            <Form.Item name="pwd" label="密码" rules={[{ required: true }]}>
              <Input.Password />
            </Form.Item>
            <Form.Item>
                <Button type="primary" htmlType="submit">提交</Button>
            </Form.Item>
        </Form>
    )
}

Login.getLayout = function getLayout(page:React.ReactNode) {
  return (
    <main>{page}</main>
  )
}