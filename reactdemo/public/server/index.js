var http = require('http')
var url = require('url')
var fs = require('fs')

http.createServer(function (req, res) {
    // 发送 HTTP 头部 
    // HTTP 状态值: 200 : OK
    // 内容类型: text/plain
    // const data = fs.readFileSync('./test.xlsx', 'utf8')
    // response.setHeader("Access-Control-Allow-Methods", "PUT,POST,DELETE")
    // response.setHeader('Access-Control-Allow-Origin', '*')
    // res.writeHead(200, {'Content-Type': 'application/octet-stream'})
    // 发送响应数据 "Hello World"
    var params = url.parse(req.url, true)
    console.log(params.path==='/getfile')
    if(params.path==='/'){
        res.writeHead(200, {'Content-Type': 'text/html'})
        fs.readFile('./index.html', function (err, data) {
            if (err) {
                return console.error(err)
            }
            res.end(data.toString())
        })
    }else if(params.path==='/getfile'){
        res.writeHead(200, {'Content-Type': 'application/octet-stream'})
        fs.readFile('./test.xlsx', 'binary', function(err, data){
            if(err){
                return
            }
            // console.log(data)
            res.end(data.toString(), 'binary')
        })
    }else{
        res.writeHead(200, {'Content-Type': 'text/plain'})
        res.end('hahahahahaha')
    }
    // var readerStream = fs.createReadStream('./test.xlsx');
    // readerStream.setEncoding('UTF8')
    // readerStream.on('data', function(chunk) {
    //     data += chunk
    // })
    // readerStream.on('end',function(){
    //     console.log(data)
    //     res.end(data)
    // })
}).listen(8888)