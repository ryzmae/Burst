# Create a Socket server that sends to the server TEST
# and receives the response from the server
#

import socket

def main():
    # Create a socket object
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    # Get the hostname
    host = "127.0.0.1"

    port = 7978

    # Connect to the server
    #
    s.connect((host, port))

    # Send the message to the server
    #
    s.sendall(b"status")

    # Receive the response from the server
    #
    data = s.recv(1024)

    s.close()

    # Print the response
    #
    print('Received', repr(data))


if __name__ == '__main__':
    main()
