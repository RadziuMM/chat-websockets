if (getCookieValue('token') && getCookieValue('id')) {
    window.location.href = '/';
}

document.getElementById('registerForm').addEventListener('submit', async function(event) {
    event.preventDefault();

    const errorMessage = document.getElementById('errorMessage');
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;
    const repeatPassword = document.getElementById('repeatPassword').value;
    if (password !== repeatPassword) {
        errorMessage.style.opacity = '1';
        errorMessage.innerHTML = "Passwords are different."
        return
    }

    const payload = {
        name: username,
        password: await hashSHA256(password)
    };

    fetch('/api/auth/register', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
    })
        .then(response => {
            if (!response.ok) {
                throw new Error(response.statusText);
            }
            return response.text();
        })
        .then(_ => {
            errorMessage.style.opacity = '0';
            errorMessage.innerHTML = "";

            window.location.href = '/login';
        })
        .catch(error => {
            errorMessage.style.opacity = '1';
            errorMessage.innerHTML = error;
            console.error('Error:', error);
        });
});

