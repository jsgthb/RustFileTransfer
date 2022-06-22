Networks 2 Project
Schaack Jeff - 020176095A
Github link: https://github.com/jsgthb/RustFileTransfer

This is a simple TCP file transfer application written in Rust.
The program consists of a server and a client which have to be run as separate instances.
The server sends the client a list of files contained in the files/server directory from which the client can then choose a file to download.
Once a file has been downloaded it appears in the files/client directory.
The program has a maximum limit of 10 MB for a file that can be transferred.

How to use:
-----------
1. Start an instance of the program using main.exe
2. Select "server" from the selection to start the server
3. Start a second instance of the program using main.exe
4. Select "client" on the second instance
5. Select which file to transfer

