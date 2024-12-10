document.getElementById('loginForm').addEventListener('submit', function(event) {
    event.preventDefault();

    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;

    const payload = {
        username: username,
        password: password
    };

    fetch('/api/login', {
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
            sessionStorage.setItem('token', data.token);
            window.location.href = '/';
        })
        .catch(error => {
            const errorMessage = document.getElementById('errorMessage');
            errorMessage.style.display = 'block';
            console.error('Error:', error);
        });
});