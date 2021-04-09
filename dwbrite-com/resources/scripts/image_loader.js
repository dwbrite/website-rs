var mediaElements = document.getElementsByClassName("media-noscript");
let length = mediaElements.length;

// for each media element
for (let i = 0; i < length; i++) {
    let data = getMediaData(i);


    // create the thumbnail element
    let thumb = document.createElement("img");

    thumb.setAttribute("src", data.thumbnail);
    thumb.setAttribute("id", "media-img-"+i);

    thumb.setAttribute("width", data.width);
    thumb.setAttribute("height", data.height);
    thumb.setAttribute("alt", data.alt);
    thumb.setAttribute("title", "click me!");

    thumb.classList.add("media-thumbnail");
    thumb.classList.add("media-content");
    if (data.pixelated) { thumb.classList.add("pxl"); }

    thumb.onclick = function() { loadMedia(i) };


    // create the anchor
    let anchor = document.createElement("a");
    anchor.classList.add("media-anchor");
    anchor.appendChild(thumb);

    // and place
    mediaElements[i].insertAdjacentElement("afterend", anchor);
}

function loadMedia(i) {
    let data = getMediaData(i);
    let src = data.file;

    let image = new Image();
    image.src = src;

    let thumb = document.getElementById("media-img-"+i);
    thumb.classList.add("media-loading");

    image.onload = function() {
        thumb.src = src;
        thumb.classList.remove("media-thumbnail");
        thumb.classList.remove("media-loading");
        thumb.setAttribute("title", data.alt);
    }
}

function getMediaData(i) {
    // get media 'source'
    let mediaElement = mediaElements[i];
    let src = mediaElement.getAttribute("data-src");

    // nab data from local storage
    let dict = JSON.parse(localStorage.getItem("mediaDict"));
    return dict[src];
}