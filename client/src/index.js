import "./styles.css"

//var host = "oh.zvs.io";
//var httpURL = "https://"+host+"/";
var host = "localhost:8080";
var httpURL = "http://"+host+"/";

window.addEventListener("load", function() {
    var input = document.getElementById("input");
    var comment = document.getElementById("comment");
    var status = document.getElementById("status");
    var line = document.getElementById("line");
    var joiner = document.getElementById("joiner");
    var leaver = document.getElementById("leaver");
    var greet = document.getElementById("greet");

    const evtSource = new EventSource("//" + host + "/sse");
    var last = evtSource.readyState;

    var datakeeper;

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

    var updateLine = function(data) {
        if (!data)
            return;
        var myname = localStorage.getItem('name');
        var entries = "";
        data.forEach(row => {
            if (myname == row.name) {
                entries += "<div class=\"flex justify-between text-gray-800 text-center bg-green-200 px-4"
                    + " py-2 m-2 rounded\">";
            } else {
                entries += "<div class=\"flex justify-between text-gray-800 text-center bg-gray-200 px-4"
                    + " py-2 m-2 rounded\">";
            }
            entries += "<div>" + row.name + "</div>";
            var seconds = (new Date() - new Date(row.join_time)) / 1000;
            var min = Math.floor(seconds/60);
            var sec = Math.floor(seconds % 60);
            entries += "  ";
            entries += "<div>" + str_pad_left(min, '0', 2) + ':' + str_pad_left(sec, '0', 2) + "</div>";
            entries += "</div>";
        });
        if (Object.keys(data).length == 0) {
            line.innerHTML = "<p class=\"text-center text-lg\">Empty!</p>";
        } else {
            line.innerHTML = entries;
        }
    }

    // https://stackoverflow.com/questions/3733227/javascript-seconds-to-minutes-and-seconds
    function str_pad_left(string, pad, length) {
        return (new Array(length+1).join(pad)+string).slice(-length);
    }

    setInterval(function() {
        updateLine(datakeeper);
    }, 500);

    evtSource.onmessage = function(evt) {
        console.log(evt);
        getStatus();
        datakeeper = JSON.parse(evt.data);
        console.log(datakeeper);
        updateLine(datakeeper);
    }

    evtSource.onopen = function() {
        getStatus();
    }

    evtSource.onerror = function() {
        getStatus();
    }

    document.getElementById("join").onclick = function() {
        let student = {
            "name": input.value,
            "comment": comment.value,
            "join_time": new Date(),
        };
        if (localStorage.getItem('name') == null) {
            fetch(httpURL+'push', {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(student)
            }).then(function(_) {
                localStorage.setItem('name', input.value);
                joiner.classList.add("hidden");
                leaver.classList.remove("hidden");
                greet.innerHTML = "Hi, " + input.value + " you are in the queue."
                return false;
            });
        }
    };

    document.getElementById("leave").onclick = function() {
        var name = localStorage.getItem('name');
        if (name != null) {
            fetch(httpURL+'leave?event='+name, {
                method: 'PUT',
            }).then(function(_) {
                localStorage.removeItem('name');
                joiner.classList.remove("hidden");
                leaver.classList.add("hidden");
                return false;
            });
        }
    };
});
