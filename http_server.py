from http.server import HTTPServer, BaseHTTPRequestHandler

HOST_NAME = ""
PORT_NUMBER = 3000

class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        print(self.client_address, self.command, self.path, dict(self.headers))
        self.send_response(200)
        self.send_header("Content-type", "text/html; charset=utf-8")
        self.end_headers()
        self.wfile.write(b"<!DOCTYPE html><html><head><meta charset=\"utf-8\"></head><body><div>Hello World!</div></body></html>")

httpd = HTTPServer((HOST_NAME, PORT_NUMBER), Handler)
httpd.serve_forever()
