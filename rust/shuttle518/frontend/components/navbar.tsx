import { useRouter } from 'next/router'
import Link from 'next/link'
import { Menu } from 'antd'

export default function Navbar(){
  const router = useRouter()
  const items = [
    {
      key: '/',
      label: <Link href="/">home</Link>
    },
    {
      key: '/image',
      label:<Link href="/image">image</Link>
    }
  ]

  return (
    <nav>
      <Menu selectedKeys={[router.pathname]} theme="dark" mode="horizontal" items={items} />
    </nav>
  )
}