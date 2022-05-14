# recordebate
Chaturbate Recorder with web UI

![Preview](https://user-images.githubusercontent.com/105115803/168447127-72264c9c-cb61-4187-8521-3325c9c85b7d.png)

# Installation
## Prerequisites
- git
- docker
## Building
```console
git clone https://github.com/PChrapos/recordebate.git
cd recordebate
docker build -t pchrapos/recordebate:latest .
```
## Run
```console
docker run -p 3000:3000 -v <absolute path to data folder>:/data -v <absolute path to config folder>:/config pchrapos/recordebate:latest
```
open http://localhost:3000 in your browser
