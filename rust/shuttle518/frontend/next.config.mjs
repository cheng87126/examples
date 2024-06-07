/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  trailingSlash: true,
  output: "export",
  distDir: "build",
  images: {
    unoptimized: true,
  },
  // https://github.com/ant-design/ant-design/issues/46053
  transpilePackages: [ "antd", "@ant-design", "rc-util", "rc-pagination", "rc-picker", "rc-notification", "rc-tooltip", "rc-tree", "rc-table" ],
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: 'https://shuttle518.shuttleapp.rs/api/:path*' // Proxy to Backend
      }
    ]
  }
};

export default nextConfig;
