const fetch = require("node-fetch");

const SERVER_URL = process.env.SERVER_URL || "http://localhost:8080";
const USERNAME = process.env.USERNAME || "ben";

fetch(`${SERVER_URL}/holdings/create?username=${USERNAME}`, {
    method: "POST",
    body: JSON.stringify({
        ticker: "AAPL",
        quantity: 5,
    }),
    headers: {
        'Content-Type': 'application/json'
    }
})
.then((resp) => {
    const contentType = resp.headers.get("content-type");
    if (contentType && contentType.indexOf("application/json") !== -1) {
        return resp.json();
    } else {
        return resp.text();
    }
})
.then(console.log)
.catch(console.error);
