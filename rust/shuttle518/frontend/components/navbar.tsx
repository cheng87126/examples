import Link from 'next/link'

export default function Navbar(){
    return (
        <div>
            <Link href="/">home</Link>
            <Link href="/image">image</Link>
        </div>
    )
}