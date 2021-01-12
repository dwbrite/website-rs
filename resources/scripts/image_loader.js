var mediaElements = document.getElementsByClassName("media-noscript");
let length = mediaElements.length;

for (i = 0; i < length; i++) {
    let btn = document.createElement("button");
    btn.innerHTML = "load media";
    btn.setAttribute("onclick", "loadMedia("+i+");");
    btn.setAttribute("id", "media-btn-"+i);
    btn.setAttribute("class", "media-btn");
    btn.setAttribute("type", "button");

    let div = document.createElement("div");
    div.setAttribute("class", "media-div");

    div.appendChild(btn);
    mediaElements[i].insertAdjacentElement("afterend", div);
}

function loadMedia(i) {
    let btn = document.getElementById("media-btn-"+i);
    btn.setAttribute("onclick", "hideMedia("+i+");");
    btn.innerHTML = "hide media";

    let div = btn.parentElement;

    // The following is a hack to get around
    // JS not seeing children of <noscript> elements.
    let temp = document.createElement("div");
    temp.innerHTML = mediaElements[i].innerHTML;
    let child = temp.firstElementChild;
    child.classList.add("media-content");

    let src = child.getAttribute("src");

    let a = document.createElement("a");
    a.setAttribute("id", "media-anchor-"+i);
    a.setAttribute("class", "media-anchor");
    a.setAttribute("href", src);
    a.appendChild(child);

    div.appendChild(a);
}

function showMedia(i) {
    let btn = document.getElementById("media-btn-"+i);
    btn.setAttribute("onclick", "hideMedia("+i+");");
    btn.innerHTML = "hide media";

    let a = document.getElementById("media-anchor-"+i);
    a.setAttribute("style", "display:block");
}

function hideMedia(i) {
    let btn = document.getElementById("media-btn-"+i);
    btn.setAttribute("onclick", "showMedia("+i+");");
    btn.innerHTML = "view media";

    let a = document.getElementById("media-anchor-"+i);
    a.setAttribute("style", "display:none");
}
