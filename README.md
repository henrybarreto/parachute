# Parachute
Parachute is a simple and fast way to delivery files to friends.

Parachute encrypts the files and store it into a volatile database, what means that any file sent to it will be lost 
after some period or when the server restarts, so, *you don't use it as a backup system !*.

These files are completely anonymous. No information about who sent the file are saved on the server, but any file what
found and considerable as inappropriate will be deleted without any warning, so, again, *you don't use parachute to 
send those kinds of files !*.

## How to use

### Upload

Sends a photo to the server.
```
parachute upload photo.jpeg
```

Sends a photo to the server with a password.
```
parachute upload -p Example123 photo.jpeg
```

Sends a photo to the server with a limit number of downloads or lifetime.
```
parachute upload -t 1 photo.jpeg
```

Sends a photo to the server with an address allowed.
```
parachute upload -a 127.0.0.1 photo.jpeg
```

### Download

Gets a file from the server.
```
parachute download xytjlxza 
```

Gets a file from the server when it has a password.
```
parachute download -p Example123 xytjlxza 
```

Gets a file from the server only when it matches to a hash.
```
parachute download -h 1a79a4d60de6718e8e5b326e338ae533 xytjlxza 
```

## Protocol

Parachute is a relative simple client-server protocol what runs over the HTTPS protocol. 

It sends a file with some flags, case signed by user, to the server. The server saves it into a volatile database. When
a download is required, it checks these flags, executing needed validation, logic and callbacks.

The Parachute client works as a HTTP client to send a file to the server.
```http
https://parachute.example/api/v1/upload
{
	"password": "",
	"times": "",
	"address" : ""
}
```

Through the client, we can also download the file from the server.
```http
https://parachute.example/api/v1/download/xytjlxza
{
	"password": "",
	"hash": ""
}
```
