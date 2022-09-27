# TinyServe

A tiny HTTP server which simply serves the contents of the folder named "public" in the same directory as the 
executable. Will automatically use index.html if a directory is visited and will used "public/404.html" if the
requested file doesn't exist.