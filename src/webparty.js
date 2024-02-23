// // https://github.com/bryc/code/blob/89ee019ab85c0cad77b99193a55d1b494c0ffdc2/jshash/experimental/cyrb53.js
// const cyrb53 = (str, seed = 0) => {
//     let h1 = 0xdeadbeef ^ seed, h2 = 0x41c6ce57 ^ seed;
//     for(let i = 0, ch; i < str.length; i++) {
//         ch = str.charCodeAt(i);
//         h1 = Math.imul(h1 ^ ch, 2654435761);
//         h2 = Math.imul(h2 ^ ch, 1597334677);
//     }
//     h1  = Math.imul(h1 ^ (h1 >>> 16), 2246822507);
//     h1 ^= Math.imul(h2 ^ (h2 >>> 13), 3266489909);
//     h2  = Math.imul(h2 ^ (h2 >>> 16), 2246822507);
//     h2 ^= Math.imul(h1 ^ (h1 >>> 13), 3266489909);
  
//     return 4294967296 * (2097151 & h2) + (h1 >>> 0);
// };

// serialize the page source
function getPageSource() {
    return '<!DOCTYPE HTML>' + '\n' + document.documentElement.outerHTML;
}

function sendUpdate() {
    let source = getPageSource();
    fetch("/update", {
        method: "PUT",
        headers: {
            "Content-Type": "text/html",
            "Authorization": "Basic " + window.localStorage.getItem("auth"),
        },
        body: source,
    });
}

function handleImages() {
    document.querySelectorAll("img").forEach(img => {
        if(img.src == window.location.href+"PARTY") { 
            // alert(1);
            img.src=""

            let id = "Party-upload-" + Math.round(Math.random()*1000);

            let fileInputElement = document.createElement("input");

            fileInputElement.type = "file";
            fileInputElement.accept = "image/*";
            fileInputElement.id = id
            fileInputElement.hidden=true;

            let labelElement = document.createElement("label");

            labelElement.htmlFor = id

            labelElement.style.backgroundColor="beige"
            labelElement.style.padding="1rem"

            labelElement.innerHTML = "<span style='font-weight:bold'>[webparty]</span> Click to Upload Image"



            if (!confirm("Click OK to upload file")) {return}

            document.body.append(fileInputElement);
            document.body.append(labelElement);

            console.log(labelElement)
            
            fileInputElement.onchange = async function() {
                // console.log(fileInputElement.files);
                let up = await fetch("/upload", {
                    method: "POST",
                    headers: {
                        "Content-Type": "blob",
                        "Authorization": "Basic " + window.localStorage.getItem("auth")
                    }
                });

                document.body.removeChild(fileInputElement);
                document.body.removeChild(labelElement);

                if(up.status == 201) {
                    let resp = await up.json();
                    img.src = resp.path;
                } else {
                    alert("Something went wrong with uploading the image");
                }
            }
            
            // document.body.removeChild(fileInputElement)



        }
    });
}

// check for changes every 100ms
// yeah there no better way to do this and I hate it but chrome devtools doesn't report changes to the dom for some reason


// Perf note: without hashing takes: ~15 micro seconds
//               with hashing takes: ~28 micro seconds

//           time between execution
//              without hashing: 4 ms
//                 with hashing: 4 ms

// let lastHash = cyrb53(getPageSource());
let lastSource = getPageSource();
function checkForChanges() {
    let source = getPageSource();
    // let hash = cyrb53(source);
    if (source !== lastSource) {
        // lastHash = hash;
        
        // check for images
        handleImages();

        lastSource = source;
        sendUpdate();
        // alert("PUT update");
    }
    // requestAnimationFrame(checkForChanges);
}

// checkForChanges();

setInterval(checkForChanges, 0);
