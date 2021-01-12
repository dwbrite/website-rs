if(sessionStorage.getItem("cachedFonts") === "true") {
    downloadFonts();
} else {
    createFontButton();
}

function downloadFonts() {
    addFontLink('/resources/fonts/Fira/fira.css');
    addFontLink('/resources/fonts/FiraCode/fira_code.css');
    addFontLink('/resources/fonts/Merriweather/merriweather.css');

    try {
        sessionStorage.setItem("cachedFonts", "true");
    } finally {
        removeFontButton();
    }
}

function addFontLink(location) {
    // Adds a <link> element for a given font's css file
    // A font's css file should contain __only__ `@font-face` rules.
    let head = document.getElementsByTagName('head')[0];
    let linkStyle = document.createElement('link');
    linkStyle.setAttribute('rel', 'stylesheet');
    linkStyle.setAttribute('type', 'text/css');
    linkStyle.setAttribute('href', location);
    head.appendChild(linkStyle);
}

function createFontButton() {
    let dlFontsBtn = document.createElement('button');
    dlFontsBtn.id = 'font-btn';
    dlFontsBtn.title = `Link font css `;
    dlFontsBtn.innerHTML = 'load fonts';

    let pageHeader = document.getElementById('page-header');
    pageHeader.innerHTML = dlFontsBtn.outerHTML + pageHeader.innerHTML;

    dlFontsBtn = document.getElementById('font-btn');
    dlFontsBtn.onclick = downloadFonts;
}

function removeFontButton() {
    let btn = document.getElementById('font-btn');
    if (btn !== null) {
        btn.remove()
    }
}
