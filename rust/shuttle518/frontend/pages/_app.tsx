import { SWRConfig } from 'swr'
import "@/styles/globals.css";
import type { AppProps } from "next/app";
import Layout from '../components/layout'
import { fetcher } from '@/utils'
import { ConfigProvider } from 'antd'
import zhCN from 'antd/locale/zh_CN'
import 'dayjs/locale/zh-cn'

export default function App({ Component, pageProps }: AppProps) {

  return (
    <SWRConfig 
      value={{
        errorRetryCount:3,
        focusThrottleInterval: 1000*60*2,
        fetcher
      }}
    >
      <ConfigProvider locale={zhCN}>
      {(Component as any).getLayout
        ? (Component as any).getLayout(<Component {...pageProps} />)
        : (
          <Layout>
            <Component {...pageProps} />
          </Layout>
        )
      }
      </ConfigProvider>
    </SWRConfig>
  )
}
