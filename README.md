# Parachute
<p align="center">
    <img src="https://user-images.githubusercontent.com/23109089/182893857-0d20157e-4d4e-4eb3-ba24-a13273723b54.png#gh-dark-mode-only" alt="WK's dark logo" />
    <img src="https://user-images.githubusercontent.com/23109089/182894503-df2aca1c-500e-4b12-b733-3dd2f60aec08.png#gh-light-mode-only" alt="WK's light logo" />
</p>

<p align="center">Parachute is a simple and fast way to delivery files to friends.</p>

<p align="center">
<img src="https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white"/>
<img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white"/>
<img src="https://img.shields.io/badge/redis-%23DD0031.svg?style=for-the-badge&logo=redis&logoColor=white"/>
</p>

Parachute encrypts the files and store it into a volatile database, what means that any file sent to it will be lost 
after some period or when the server restarts, so, *you don't use it as a backup system!*.

These files are completely anonymous. No information about who sent the file are saved on the server, but any file what
found and considerable as inappropriate will be deleted without any warning, so, again, *you don't use parachute to 
send those kinds of files!*.

## How to use

> NOTICE: The CLI interface isn't implemented yet. Currently, you can run the `server` and the `client` as standalone binaries.

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
parachute download 00000000-0000-0000-0000-000000000000 
```

Gets a file from the server when it has a password.
```
parachute download -p Example123 00000000-0000-0000-0000-000000000000 
```

Gets a file from the server only when it matches to a hash.
```
parachute download -h 1a79a4d60de6718e8e5b326e338ae533 00000000-0000-0000-0000-000000000000 
```

## Protocol

Parachute is a relative simple client-server protocol what runs over the TCP protocol. 

It sends a file with some flags, case signed by user, to the server. The server saves it into a volatile database to
be downloaded when required. It checks the flags, executing needed validation, logic and callbacks when that download
is requested.
