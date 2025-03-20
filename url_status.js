const fetch = require("node-fetch");

async function checkURL(url) {
    try {
        const response = await fetch(url);
        console.log(`✅ ${url} is online! Status: ${response.status}`);
    } catch (error) {
        console.log(`❌ ${url} is offline!`);
    }
}

checkURL("https://google.com");
