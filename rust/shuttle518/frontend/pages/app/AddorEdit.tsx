import { forwardRef, useImperativeHandle } from 'react'
import useSWR from 'swr'
import dayjs from 'dayjs'
import { Form, Input, DatePicker, Spin } from 'antd'
import { getFundName, addOrUpdateFund } from '@/api/app'
import type { GetProps } from 'antd'

type FieldType = {
  code: string
  fund_name: string
  buy_date: dayjs.Dayjs
  price: string
  amount: string
  tranche: string
}
interface Props{
  editId?:number
  cb?:()=>void
}

type RangePickerProps = GetProps<typeof DatePicker.RangePicker>
const disabledDate: RangePickerProps['disabledDate'] = (current) => {
  return current && current > dayjs().endOf('day')
}

export default forwardRef(function AddorEdit({editId,cb}:Props,ref){
  const { data={}, isLoading } = useSWR(editId?{url:'/api/getFund',data:{id:editId}}:null,{revalidateOnFocus:false})
  const [form] = Form.useForm()

  const onFinish = async function(val:FieldType){
    const {
      buy_date,
      price,
      amount,
      tranche,
      ...rest
    } = val
    await addOrUpdateFund({
      id:editId,
      buy_date:buy_date.format('YYYY-MM-DD'),
      price:+price,
      amount:+amount,
      tranche:+tranche,
      ...rest
    })
    cb?.()
  }

  const onBlur = async function(e:React.FocusEvent<HTMLInputElement, Element>){
    const res = await getFundName(e.target.value)
    form.setFieldValue('fund_name',res.fd_name)
  }

  useImperativeHandle(ref, () => {
    return {
      submit:function(){
        form.submit()
      }
    }
  }, [])
  if(isLoading) return <div className="text-center"><Spin /></div>
  const {
    buy_date,
    ...rest
  } = data
  return (
    <Form
      form={form}
      initialValues={editId?{ buy_date:dayjs(buy_date), ...rest }:{}}
      onFinish={onFinish}
      autoComplete="off"
    >
      <Form.Item<FieldType>
        label="基金代码"
        name="code"
        rules={[{ required: true, message: '请输入基金代码' }]}
      >
        <Input placeholder="请输入基金代码" onBlur={onBlur} />
      </Form.Item>
      <Form.Item<FieldType>
        label="基金名称"
        name="fund_name"
        rules={[{ required: true, message: '请输入基金名称' }]}
      >
        <Input placeholder="请输入基金名称" />
      </Form.Item>
      <Form.Item<FieldType>
        label="买入日期"
        name="buy_date"
        rules={[{ required: true, message: '请选择买入日期' }]}
      >
        <DatePicker disabledDate={disabledDate} format="YYYY-MM-DD" className="w-full" />
      </Form.Item>
      <Form.Item<FieldType>
        label="确认金额"
        name="amount"
        rules={[{ required: true, message: '请输入确认金额' }]}
      >
        <Input placeholder="请输入确认金额" />
      </Form.Item>
      <Form.Item<FieldType>
        label="确认净值"
        name="price"
        rules={[{ required: true, message: '请输入确认净值' }]}
      >
        <Input placeholder="请输入确认净值" />
      </Form.Item>
      <Form.Item<FieldType>
        label="确认份额"
        name="tranche"
        rules={[{ required: true, message: '请输入确认份额' }]}
      >
        <Input placeholder="请输入确认份额" />
      </Form.Item>
    </Form>
  )
})