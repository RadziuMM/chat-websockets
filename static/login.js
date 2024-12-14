if (getCookieValue('token') && getCookieValue('id')) {
    window.location.href = '/';
}

document.getElementById('loginForm').addEventListener('submit', async function(event) {
    event.preventDefault();

    const errorMessage = document.getElementById('errorMessage');
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;

    const payload = {
        name: username,
        password: await hashSHA256(password)
    };

    fetch('/api/auth/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Invalid credentials');
            }
            return response.json();
        })
        .then(data => {
            document.cookie = `id=${data.id}; path=/; SameSite=Strict;`;
            document.cookie = `name=${data.name}; path=/; SameSite=Strict;`;
            document.cookie = `token=${data.token}; path=/; SameSite=Strict;`;
            window.location.href = '/';
        })
        .catch(error => {
            errorMessage.style.opacity = '1';
            errorMessage.innerHTML = error;
            console.error('Error:', error);
        });
});