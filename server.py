import sys
import logging
from http.server import SimpleHTTPRequestHandler, HTTPServer

# Clear the log file at the beginning of the script
with open('server.log', 'w') as log_file:
    log_file.write('')

# Configure logging to write to a file
logging.basicConfig(
    filename='server.log',
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s'
)

# Remove caching
class NoCacheHTTPRequestHandler(SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Cache-Control", "no-store, no-cache, must-revalidate, max-age=0")
        self.send_header("Expires", "0")
        self.send_header("Pragma", "no-cache")
        super().end_headers()

    def log_message(self, format, *args):
        # Log requests to the log file
        logging.info("%s - - [%s] %s\n" %
                     (self.client_address[0],
                      self.log_date_time_string(),
                      format % args))

if __name__ == "__main__":
    # Check for a port argument
    if len(sys.argv) > 1:
        try:
            port = int(sys.argv[1])
        except ValueError:
            logging.info(f"Invalid port number: `{port}' Please provide a valid number")
            print(f"Invalid port number: `{port}' Please provide a valid number")
            sys.exit(1)

    else:
        logging.info(f"Server need a port to start")
        print(f"Server need a port to start")
        sys.exit(1)

    server = HTTPServer(('0.0.0.0', port), NoCacheHTTPRequestHandler)
    print(f"Serving on port {port}")
    logging.info(f"Server started on port {port}")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("Server is shutting down...")
        logging.info("Server stopped")
        server.server_close()