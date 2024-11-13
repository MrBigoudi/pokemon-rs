from http.server import SimpleHTTPRequestHandler, HTTPServer
import logging
import socket
import sys

# Configure logging to output to server_output.log and clear it at startup
logging.basicConfig(
    filename="server_output.log",
    filemode="w",  # Clear the file at startup
    format="%(asctime)s - %(levelname)s - %(message)s",
    level=logging.INFO
)

class CustomHandler(SimpleHTTPRequestHandler):
    def log_message(self, format, *args):
        # Redirect all regular log messages to server_output.log
        logging.info(format % args)

    def log_error(self, format, *args):
        # Redirect error messages to server_output.log
        logging.error(format % args)

# Redirect uncaught exceptions to the log file
def log_uncaught_exceptions(exctype, value, tb):
    logging.critical("Uncaught exception", exc_info=(exctype, value, tb))

# Assign our custom exception handler
sys.excepthook = log_uncaught_exceptions

if __name__ == "__main__":
    try:
        server = HTTPServer(('localhost', 8080), CustomHandler)
        logging.info("Starting server on port 8080...")
        server.serve_forever()
    except KeyboardInterrupt:
        logging.info("Server interrupted, shutting down.")
    except socket.error as e:
        logging.error(f"Socket error: {e}")
    finally:
        logging.info("Shutting down the server.")