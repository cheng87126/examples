import { SWRConfig } from 'swr'
import "@/styles/globals.css";
import type { AppProps } from "next/app";
import Layout from '../components/layout'
import { fetcher } from '@/utils'

export default function App({ Component, pageProps }: AppProps) {

  return (
    <SWRConfig 
      value={{
        focusThrottleInterval: 1000*60*2,
        fetcher
      }}
    >
      {(Component as any).getLayout
        ? (Component as any).getLayout(<Component {...pageProps} />)
        : (
          <Layout>
            <Component {...pageProps} />
          </Layout>
        )
      }
    </SWRConfig>
  )
}
