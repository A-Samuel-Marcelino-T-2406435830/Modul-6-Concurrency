# *Module 6 - Concurrency*
# Reflection 1
The output in my terminal is as given below:
```
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "Connection: keep-alive",
    "Cache-Control: max-age=0",
    "sec-ch-ua: \"Google Chrome\";v=\"147\", \"Not.A/Brand\";v=\"8\", \"Chromium\";v=\"147\"",
    "sec-ch-ua-mobile: ?0",
    "sec-ch-ua-platform: \"Windows\"",
    "Upgrade-Insecure-Requests: 1",
    "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-User: ?1",
    "Sec-Fetch-Dest: document",
    "Accept-Encoding: gzip, deflate, br, zstd",
    "Accept-Language: en-US,en;q=0.9,id;q=0.8",
    "Cookie: csrftoken=GcID3RUnJuqCbalshdW9gaXSM5C9Z33n",
]
```
This appears to be a http request sent from my browser to this web server. The request comes in this format: 
```
Method Request-URI HTTP-Verison CRLF
headers CRLF
message-body
```
- The first line `"GET / HTTP/1.1"` tells us about what the client is requesting. In this case the client uses the `GET` http method with the URI being `/` and the version being `HTTP/1.1`. 
- GET requests have no message-body, so the rest of the information given are headers.

We get this kind of output thanks to the `BufReader` in the `handle_connection` method. It allows us to wrap the raw `TcpStream` inside a buffer for efficient reading. The stream is read line-by-line, then we use `.unwrap()` on each line to extract raw string from the network readings. Since the HTTP request separates headers from the message-body with a new line, we stop reading until we hit that new line. The collected information is then formatted into the vector `http_request`. The vector is then printed and we got the output given above.

