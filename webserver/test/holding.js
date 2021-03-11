const fetch = require("node-fetch");

const SERVER_URL = process.env.SERVER_URL ?? "http://localhost:8080";
const USERNAME = process.env.USERNAME ?? "ben";
const MODE = process.env.MODE ?? "buy";

function parseResponse(resp) {
    const contentType = resp.headers.get("content-type");
    if (contentType && contentType.indexOf("application/json") !== -1) {
        return resp.json();
    } else {
        return resp.text();
    }
}

async function run() {
    const commonJSON = {
        method: "POST",
        body: JSON.stringify({
            ticker: "AAPL",
            quantity: 5,
        }),
        headers: {
            'Content-Type': 'application/json'
        }
    };

    if (MODE === "buy") {
        let resp = await fetch(`${SERVER_URL}/holdings/buy?username=${USERNAME}`, commonJSON);
    
        console.log(await parseResponse(resp));
    } else if (MODE === "sell") {
        let resp = await fetch(`${SERVER_URL}/holdings/sell?username=${USERNAME}`, commonJSON);
    
        console.log(await parseResponse(resp));
    } else {
        console.log(`Did not recognize mode: ${MODE}`);
    }
}

run().then(() => {
    console.log("finished");
}).catch(console.error);
