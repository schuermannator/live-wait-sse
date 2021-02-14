# Live Waitqueue
![CI](https://github.com/schuermannator/live-wait-sse/workflows/CI/badge.svg)

This is a queueing mechanism for my office hours. In Spring 2020, classes became virtual for a
significant part of the semester; I needed an effective means to order those joining my online office
hours. 

# Usage
The application is live at [https://oh.zvs.io](https://oh.zvs.io). Simply go there and join the queue.  
For TA's only: there is a Rust client which allows for administration of the queue via a CLI.

## Building
cli:
```
docker build -t docker.pkg.github.com/schuermannator/live-wait-sse/live-wait-cli:latest -f client.Dockerfile .
```

server:
```
$ pushd client && npm install && npm run build
$ DOCKER_BUILDKIT=1 docker build -t live-wait .
```

## Todo
- [x] Fix SSE (impl non-closing `Read` etc.)
- [ ] Make checker for removing local storage if not in waitqueue after awaiting response
      (kick people from client when removed)
- [ ] Add 'current' tab in Rust client (the last popped student + info)
- [ ] Add times to Rust client
- [ ] Make you're up next alert and/or the current student with how long they have been in OH
- [ ] Implement enter key for joining queue
- [ ] Add hours parsing for waittimes

## Rocket-SSE
SSE implementation from [Jeb Rosen](https://git.jebrosen.com/jeb/rocket-rooms). 
