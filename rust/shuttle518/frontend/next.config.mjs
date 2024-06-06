/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  trailingSlash: true,
  output: "export",
  distDir: "build",
  images: {
    unoptimized: true,
  }
};

export default nextConfig;
