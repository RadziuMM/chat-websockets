document.getElementById("name-message").innerHTML = getCookieValue("name");

function enterChat(chatId) {
    alert("Entering chat: " + chatId);
}
function deleteChat(chatId) {
    alert("Deleting chat: " + chatId);
}

function openPopup() {
    const popup = document.getElementById("popup");
    popup.style.display = "flex";
}

function closePopup() {
    const popup = document.getElementById("popup");
    popup.style.display = "none";
}

function submitChat() {
    const roomName = document.getElementById("room-name").value;
    if (!roomName.trim()) {
        alert("Room name cannot be empty!");
        return;
    }
    alert("New chat created: " + roomName);

    closePopup();
}