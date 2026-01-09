from socket import * # import socket module
import sys # In order to terminate the program

serverSocket = socket(AF_INET, SOCK_STREAM)
# Prepare a server socket
# Fill in start
#
# Fill in end

while True:
    # Establish the connection
    print('Ready to serve...')
    connectionSocket, address = # Fill in start # Fill in end
    try:
        message = # Fill in start # Fill in end
        # Send one HTTP header line into socket
        # Fill in start
        #
        # Fill in end
        # Send the conten of the requested file to the client
        for i in range(0, len(outputdata)):
            connectionSocket.send(outputdata[i].encode())
        connectionSocket.send("\r\n".encode())

        connectionSocket.close()
    except IOError:
        # Send response message for file not found
        # Fill in start
        #
        # Fill in end

        # Close client socket
        # Fill in start
        #
        # Fill in end
    serverSocket.close()
    sys.exit() # Terminate the program after sendin the corresponding data
