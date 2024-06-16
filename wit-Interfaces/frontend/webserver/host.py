from webserver_host import Root
from wasmtime import Config, Engine, Store, WasiConfig

import http.server
import socketserver

PORT = 8000


config = Config()
engine = Engine(config)
wasi_config = WasiConfig()
store = Store(engine)
webserver = Root(store)

Handler = http.server.SimpleHTTPRequestHandler
with socketserver.TCPServer(("", PORT), Handler) as httpd:
    print(f"Serving at port {PORT}")
    httpd.serve_forever()
