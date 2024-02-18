// https://github.com/bryc/code/blob/89ee019ab85c0cad77b99193a55d1b494c0ffdc2/jshash/experimental/cyrb53.js
const cyrb53 = (str, seed = 0) => {
    let h1 = 0xdeadbeef ^ seed, h2 = 0x41c6ce57 ^ seed;
    for(let i = 0, ch; i < str.length; i++) {
        ch = str.charCodeAt(i);
        h1 = Math.imul(h1 ^ ch, 2654435761);
        h2 = Math.imul(h2 ^ ch, 1597334677);
    }
    h1  = Math.imul(h1 ^ (h1 >>> 16), 2246822507);
    h1 ^= Math.imul(h2 ^ (h2 >>> 13), 3266489909);
    h2  = Math.imul(h2 ^ (h2 >>> 16), 2246822507);
    h2 ^= Math.imul(h1 ^ (h1 >>> 13), 3266489909);
  
    return 4294967296 * (2097151 & h2) + (h1 >>> 0);
};

// serialize the page source
function getPageSource() {
    return '<!DOCTYPE HTML>' + '\n' + document.documentElement.outerHTML;
}

function sendUpdate() {
    let source = getPageSource();
    fetch('http://localhost:8000/update', {
        method: 'PUT',
        headers: {
            'Content-Type': 'text/html',
            'Authorization': 'Basic ' + window.localStorage.getItem('auth'),
        },
        body: source,
    });
}

// check for changes every 100ms
// yeah there no better way to do this and I hate it but chrome devtools doesent report changes to the dom for some reason

let lastHash = cyrb53(getPageSource());
function checkForChanges() {
    let source = getPageSource();
    let hash = cyrb53(source);
    if (hash !== lastHash) {
        lastHash = hash;
        sendUpdate();
        // alert("PUT update");
    }
}

setInterval(checkForChanges, 100);
