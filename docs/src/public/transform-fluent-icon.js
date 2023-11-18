(function () {
    function load() {
        var elements = document.querySelectorAll("i.fluent-icon");
        elements.forEach(function (e) {
            var icon = e.getAttribute("data-icon");
            var si = icon.search(/\d+/g);
            const mi = icon.substring(si).search(/(Regular|Filled)/g);
            if (si === -1 || mi === -1) {
                console.error("Invalid icon name: " + icon);
            }
            var name = icon
                .substring(0, si)
                .replace(/([A-Z])/g, " $1")
                .trim();
            var snake = name.replace(/ /g, "_").toLowerCase();
            var size = icon.substring(si, si + mi);
            var modifier = icon.substring(si + mi);
            var file = `ic_fluent_${snake}_${size}_${modifier.toLowerCase()}`;
            var url = `https://raw.githubusercontent.com/microsoft/fluentui-system-icons/main/assets/${encodeURIComponent(
                name,
            )}/SVG/${file}.svg`;
            fetch(url)
                .then(function (r) {
                    r.text().then(function (t) {
                        e.innerHTML = t;
                    });
                })
                .catch(function (e) {
                    console.error(e);
                });
        });
    }
    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", load);
    } else {
        load();
    }
})();
