var body;
window.addEventListener('load', function () {
    body = document.getElementsByTagName("body")[0];
})

window.addEventListener("scroll", (event) => {
    body.style = `background-position: 0 ${this.scrollY * 0.75}px`;
});