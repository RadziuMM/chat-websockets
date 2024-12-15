const params = new URLSearchParams(window.location.search);

const name = getCookieValue("name");
const roomId = params.get("id");
if (!roomId) {
    window.location.href = '/';
}

fetch(`/api/room/${roomId}`, {
    method: 'GET',
    headers: {
        'Content-Type': 'application/json'
    },
}).then(response => {
    if (!response.ok) {
        window.location.href = '/';
    }
    return response.json();
}).then(data => {
    const container = document.getElementById("chat-container");
    const room = document.getElementById('room-name');
    room.innerHTML = data.name;

    data.messages.forEach(message => {
        const messageElement = document.createElement("div");
        messageElement.classList.add("message");

        if (message.username === name) {
            messageElement.classList.add("user");
        } else {
            messageElement.classList.add("other")
        }

        messageElement.innerHTML = `
            <div class="message-wrapper">
                <div class="username">${message.username} </div>
                <div class="text">${message.content}</div>
            </div>
            <div class="message-date">${new Date(message.date).toLocaleString()}</div>
        `;

        container.appendChild(messageElement);
        container.scrollTo({
            top: container.scrollHeight,
            behavior: "smooth"
        });
    })

}).catch(error => {
    console.error('Error:', error);
});

const socket = new WebSocket(`ws://localhost:3000/api/message/send?id=${roomId}`)
const socketGet = new WebSocket(`ws://localhost:3000/api/message/get?id=${roomId}`)

socket.addEventListener("open", () => {
    console.log("Connected to the WebSocket server.");
});
socketGet.addEventListener("open", () => {
    console.log("Connected to the WebSocket server.");
});
socket.addEventListener("error", (error) => {
    console.error("WebSocket error:", error);
});
socketGet.addEventListener("error", (error) => {
    console.error("WebSocket error:", error);
});

socket.addEventListener("close", () => {
    console.log("WebSocket connection closed.");
});
socketGet.addEventListener("close", () => {
    console.log("WebSocket connection closed.");
});

socketGet.addEventListener("message", (event) => {
    let message = {};
    try {
        message = JSON.parse(event.data);
    } catch (err) {
        throw Error(event.data)
    }

    const container = document.getElementById("chat-container");
    const messageElement = document.createElement("div");
    messageElement.classList.add("message");

    if (message.username === name) {
        messageElement.classList.add("user");
    } else {
        messageElement.classList.add("other")
    }

    messageElement.innerHTML = `
        <div class="message-wrapper">
            <div class="username">${message.username} </div>
            <div class="text">${message.content}</div>
        </div>
        <div class="message-date">${new Date(message.date).toLocaleString()}</div>
    `;

    container.appendChild(messageElement);
    container.scrollTo({
        top: container.scrollHeight,
        behavior: "smooth"
    });
});

function sendMessage(event) {
    event.preventDefault();

    const messageInput = document.getElementById("message-input");
    const messageText = messageInput.value.trim();

    if (!messageText) {
        return;
    }

    socket.send(messageText);
    messageInput.value = "";
}

function back() {
    window.location.href = "/";
}