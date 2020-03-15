import "./styles.css"

var httpURL = "http://localhost:8080/";

window.addEventListener("load", function(evt) {
    //var output = document.getElementById("output");
    var input = document.getElementById("input");
    var status = document.getElementById("status");
    var line = document.getElementById("line");
    var joiner = document.getElementById("joiner");
    var leaver = document.getElementById("leaver");
    var greet = document.getElementById("greet");
    //var ws;

    //var print = function(message) {
    //    var d = document.createElement("div");
    //    d.innerHTML = message;
    //    output.appendChild(d);
    //}

    //ws = new WebSocket(wsURL);
    const evtSource = new EventSource("//localhost:8080/sse");

    //console.log(evtSource.withCredentials); (false rn)
    //console.log(evtSource.url);

    if (localStorage.getItem('name') != null) {
        joiner.classList.add("hidden");
        leaver.classList.remove("hidden");
        greet.innerHTML = "Hi, " + this.localStorage.getItem('name') + " you are in the queue.";
    } else {
        joiner.classList.remove("hidden");
        leaver.classList.add("hidden");
    }

    var getStatus = function() {
        console.log(evtSource.readyState);
        var stat = ""
        switch(evtSource.readyState) {
            case 1:
                stat += "OPEN"
                break;
            default:
                stat += "CLOSED"
        }
        status.innerHTML = "Connection: " + stat;
        if (localStorage.getItem('name') == null) {
            joiner.classList.remove("hidden");
            leaver.classList.add("hidden");
        }
    }

    var updateLine = function(linedata) {
        var data = linedata;
        if (!data)
            return;
        console.log(data)
        var entries = "";
        var myname = localStorage.getItem('name');
        for (name in data) {
            if (myname == data[name]) {
                entries += "<div class=\"text-gray-800 text-center bg-green-200 px-4"
                    + " py-2 m-2 rounded\">";
            } else {
                entries += "<div class=\"text-gray-800 text-center bg-gray-200 px-4"
                    + " py-2 m-2 rounded\">";
            }
            entries += data[name];
            entries += "</div>";
        }
        if (Object.keys(data).length == 0) {
            line.innerHTML = "<p class=\"text-center text-lg\">Empty!</p>";
        } else {
            line.innerHTML = entries;
        }
    }

    evtSource.onmessage = function(evt) {
        //print("OPEN");
        console.log("GOT EVT");
        console.log(evt);
        console.log(JSON.parse(evt.data));
        getStatus();
        updateLine(JSON.parse(evt.data));
    }
    //ws.onclose = function(evt) {
    //    //print("CLOSE");
    //    getStatus();
    //    ws = null;
    //}
    //ws.onmessage = function(evt) {
    //    //print("RESPONSE: " + evt.data);
    //    getStatus();
    //    updateLine(JSON.parse(evt.data));
    //}
    //ws.onerror = function(evt) {
    //    //print("ERROR: " + evt.data);
    //    getStatus();
    //}
    document.getElementById("join").onclick = function(evt) {
        if (localStorage.getItem('name') == null) {
            //print("SEND: " + input.value);
            //ws.send(input.value);
            fetch(httpURL+'push?event='+input.value, {
                method: 'PUT',
            });
            // move this
            localStorage.setItem('name', input.value);
            joiner.classList.add("hidden");
            leaver.classList.remove("hidden");
            greet.innerHTML = "Hi, " + input.value + " you are in the queue."
            return false;
        }
    };

    document.getElementById("leave").onclick = function(evt) {
        //if (!ws) {
            //return false;
        //}
        var name = localStorage.getItem('name');
        if (name != null) {
            //print("Leave: " + name);
            fetch(httpURL+'leave?event='+name, {
                method: 'PUT',
            }).then(function(response) {
                response.text().then(function(text) {
                    //console.log(text);
                });
            });
            // move this
            localStorage.removeItem('name');
            joiner.classList.remove("hidden");
            leaver.classList.add("hidden");
            return false;
        }
    };
});
