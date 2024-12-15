document.getElementById("name-message").innerHTML = getCookieValue("name");

function enterChat(chatId) {
    window.location.href = `/room?id=${chatId}`;
}

function deleteChat(chatId) {
    alert("Deleting chat: " + chatId);
}

const loadChatRooms = () => fetch('/api/room', {
    method: 'GET',
    headers: {
        'Content-Type': 'application/json'
    },
}).then(response => {
    if (!response.ok) {
        throw new Error(response.statusText);
    }
    return response.json();
}).then(data => {
    const chatRooms = document.getElementById("chats-rooms");
    const holder = document.createElement("tbody");

    data.forEach(room => {
        const tr = document.createElement("tr");
        tr.innerHTML = `
            <tr>
                <td>${room.name}</td>
                <td class="actions">
                    <div class="actions-wrapper">
                        <button class="enter" onclick="enterChat('${room.id}')">Enter</button>
                        <button class="delete" onclick="deleteChat('${room.id}')">Delete</button>
                    </div>
                </td>
            </tr>
        `
        holder.appendChild(tr);
    })

    chatRooms.innerHTML = holder.innerHTML;
}).catch(error => {
    console.error('Error:', error);
});
loadChatRooms();

function openPopup() {
    const popup = document.getElementById("popup");
    popup.style.display = "flex";
}

function closePopup() {
    const popup = document.getElementById("popup");
    popup.style.display = "none";
}

const socket = new WebSocket(`ws://localhost:3000/api/room/send`)
const socketGet = new WebSocket(`ws://localhost:3000/api/room/get`)

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
    loadChatRooms().catch(error => console.error(error));
});

function submitChat() {
    const roomName = document.getElementById("room-name").value;
    if (!roomName.trim()) {
        return;
    }

    socket.send(roomName);
    document.getElementById("room-name").value = "";
    closePopup();
    loadChatRooms().catch(error => console.error(error));
}