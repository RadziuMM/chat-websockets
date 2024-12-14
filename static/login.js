if (sessionStorage.getItem('token') && sessionStorage.getItem('id')) {
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
            sessionStorage.setItem('id', data?.id);
            sessionStorage.setItem('name', data?.name);
            sessionStorage.setItem('token', data?.token);
            window.location.href = '/';
        })
        .catch(error => {
            errorMessage.style.opacity = '1';
            errorMessage.innerHTML = error;
            console.error('Error:', error);
        });
});