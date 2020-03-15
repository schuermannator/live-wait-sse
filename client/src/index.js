import "./styles.css"

var httpURL = "http://localhost:8080/";

window.addEventListener("load", function(evt) {
    var input = document.getElementById("input");
    var status = document.getElementById("status");
    var line = document.getElementById("line");
    var joiner = document.getElementById("joiner");
    var leaver = document.getElementById("leaver");
    var greet = document.getElementById("greet");

    const evtSource = new EventSource("//localhost:8080/sse");
    var last = evtSource.readyState;

    if (localStorage.getItem('name') != null) {
        joiner.classList.add("hidden");
        leaver.classList.remove("hidden");
        greet.innerHTML = "Hi, " + this.localStorage.getItem('name') + " you are in the queue.";
    } else {
        joiner.classList.remove("hidden");
        leaver.classList.add("hidden");
    }

    var getStatus = function() {
        var stat = ""
        switch(evtSource.readyState + last) {
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
        last = evtSource.readyState;
    }

    var updateLine = function(linedata) {
        var data = linedata;
        if (!data)
            return;
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
        getStatus();
        updateLine(JSON.parse(evt.data));
    }

    evtSource.onopen = function(evt) {
        getStatus();
    }

    evtSource.onerror = function(evt) {
        getStatus();
    }

    document.getElementById("join").onclick = function(evt) {
        if (localStorage.getItem('name') == null) {
            fetch(httpURL+'push?event='+input.value, {
                method: 'PUT',
            }).then(function(response) {
                localStorage.setItem('name', input.value);
                joiner.classList.add("hidden");
                leaver.classList.remove("hidden");
                greet.innerHTML = "Hi, " + input.value + " you are in the queue."
                return false;
            });
        }
    };

    document.getElementById("leave").onclick = function(evt) {
        var name = localStorage.getItem('name');
        if (name != null) {
            fetch(httpURL+'leave?event='+name, {
                method: 'PUT',
            }).then(function(response) {
                localStorage.removeItem('name');
                joiner.classList.remove("hidden");
                leaver.classList.add("hidden");
                return false;
            });
        }
    };
});
