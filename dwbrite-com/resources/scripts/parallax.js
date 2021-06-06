var isMobile = /iPhone|iPad|iPod|Android/i.test(navigator.userAgent);

var body;

function defineBody() {
    body = document.getElementsByTagName("body")[0];
}

window.addEventListener('load', defineBody())

if (!window.chrome || !isMobile) {
    window.addEventListener("scroll", (event) => {
        if (body === undefined) {
            defineBody()
        }

        body.style = `background-position: 0 ${this.scrollY * 0.75}px`;
    });
}
