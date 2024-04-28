import socket


def main():
    host = "192.168.0.217:8080"
    port = 8080
    message = '{"message_type": "test", "operation": "Insert", "name": "Bob"}'
    send_message(host, port, message)

    message = '{"message_type": "test", "operation": "Insert", "name": "Alice"}'
    send_message(host, port, message)

    message = '{"message_type": "test", "operation": "Delete", "name": "Bob"}'
    send_message(host, port, message)


def send_message(host, port, message):
    # Create a TCP socket
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        # Connect to the server
        s.connect((host, port))

        # Send the message
        s.sendall(message.encode())

    print("Message sent to server:", message)


if __name__ == "__main__":
    main()
