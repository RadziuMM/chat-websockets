document.getElementById('registerForm').addEventListener('submit', function(event) {
    event.preventDefault();
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;
    const repeatPassword = document.getElementById('repeatPassword').value;

    const errorMessage = document.getElementById('errorMessage');
    if (password !== repeatPassword) {
        errorMessage.style.opacity = '1';
        errorMessage.innerHTML = "Passwords are different."
        return
    }

    const payload = {
        username: username,
        password: password
    };

    fetch('/api/register', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
    })
        .then(response => {
            if (!response.ok) {
                console.log(response)
                throw new Error(response.statusText);
            }
            return response.text();
        })
        .then(data => {
            console.log(data)
            // window.location.href = '/login';
        })
        .catch(error => {
            errorMessage.style.opacity = '1';
            errorMessage.innerHTML = error
            console.error('Error:', error);
        });
});