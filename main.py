import socket

class SocketConnection():
    def __init__(
        self,
        host: str,
        port: int
    ) -> None:
        self.s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.host = host
        self.port = port

    def exec(self, message: str) -> str:
        self.s.connect((self.host, self.port))
        self.s.sendall(message.encode())
        data = self.s.recv(1024)
        self.s.close()

        return data.decode()

    def __del__(self):
        self.s.close()

if __name__ == '__main__':
    SocketConnection("127.0.0.1", 7978).exec("Hello, World!")
