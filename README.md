# Live Waitqueue

This is a queueing mechanism for my office hours. In Spring 2020, classes became virtual for a
significant part of the semester; I needed an effective means to order those joining my online office
hours. 

# Usage
The application is live at [https://oh.zvs.io](https://oh.zvs.io). Simply go there and join the queue.  
For TA's only: there is a Rust client which allows for administration of the queue via a CLI.

## Todo
- [ ] Fix SSE (impl non-closing `Read` etc.)
- [ ] Make checker for removing local storage if not in waitqueue after awaiting response
- [ ] Add 'current' tab in Rust client
- [ ] Add times to Rust client
- [ ] Make you're up next alert?
- [ ] Implement enter key for joining queue
