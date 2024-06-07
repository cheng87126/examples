import useSWR from 'swr'

export default function Home() {
  const { data, error, isLoading } = useSWR({url:'/api/getUrls'})
  return (
    <div>index</div>
  );
}
