import socket
import threading
import time
import json

class IRRL:

    def __init__(self, host="localhost", port=5001):
        self.host = host
        self.port = port

        self.server_socket = None
        self.conn = None
        self.addr = None

        self.listener_thread = None
        self.running = False
        self.callback = None

        self.irrlConnect()

    def irrlConnect(self):
        if self.running:
            return

        self.server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.server_socket.bind((self.host, self.port))
        self.server_socket.listen()

        print("--- ::: Listening for IRRL events ::: ---")

        self.conn, self.addr = self.server_socket.accept()
        print(f"--- ::: Connected by {self.addr} ::: ---")

        self.running = True

    def listen(self, callback):
        self.callback = callback

        def _listen_loop():

            buffer = ""

            while self.running:
                try:
                    data = self.conn.recv(1024)

                    if not data:
                        break

                    buffer += data.decode("utf-8")

                    while "\n" in buffer:
                        line, buffer = buffer.split("\n", 1)
                        line = line.strip()

                        if not line:
                            continue

                        try:
                            event = json.loads(line)
                            self.callback(event)
                        except json.JSONDecodeError as e:
                            print("JSON parse error:", e, "->", line)

                except Exception as e:
                    print("Listener error:", e)
                    break
        self.listener_thread = threading.Thread(
            target=_listen_loop,
            daemon=True
        )
        self.listener_thread.start()

    def irrlDisconnect(self):
        if not self.running:
            return

        self.running = False

        try:
            if self.conn:
                self.conn.close()
        except:
            pass

        try:
            if self.server_socket:
                self.server_socket.close()
        except:
            pass

        print("--- ::: IRRL Disconnected ::: ---")

    def actionsRpa(self, actions):

        if not self.running:
            return

        stop_event = threading.Event()

        def actionCallback(data):

            try:

                # If is_final is received, stop listening and disconnect
                if data.get("is_final"):
                    stop_event.set()
                    self.irrlDisconnect()
                    return

                # Get index of action to perform
                index = data.get("index_of_action")

                # Perform action if index is valid
                if index is not None and index < len(actions):
                    actions[index]()

            except Exception as e:
                print("Action error:", e)
                stop_event.set()
                self.irrlDisconnect()

        # Start listening for events
        self.listen(actionCallback)

        # Wait until stop_event is set
        stop_event.wait()