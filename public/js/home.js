fetch("/count").then(response => response.json()).then(json => {
    if (json.count > 0) {
        document.getElementById("count").innerHTML = `${json.count.toLocaleString("en")} statistics generated`;
    }
});