import { useRouter } from 'next/router'
import { Form, Input, Button } from 'antd'

export default function Login(){
  const router = useRouter()
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
      // console.log(res)
      if(res.status===200){
        router.push('/')
      }
  }

  return (
    <Form
      form={form}
      onFinish={onFinish}
      labelCol={{span: 4}}
      className="w-3/12 mt-60 mx-auto"
    >
      <Form.Item name="user_name" label="用户名" rules={[{ required: true }]}>
        <Input placeholder="用户名" />
      </Form.Item>
      <Form.Item name="pwd" label="密码" rules={[{ required: true }]}>
        <Input.Password placeholder="密码" />
      </Form.Item>
      <Form.Item className="text-right">
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