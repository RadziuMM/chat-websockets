if (!sessionStorage.getItem('token') || !sessionStorage.getItem('id')) {
    sessionStorage.removeItem("token")
    sessionStorage.removeItem("id")
    window.location.href = '/login';
}

fetch('/api/auth/me', {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json'
    },
    body: JSON.stringify({
        id: sessionStorage.getItem('id'),
        token: sessionStorage.getItem('token'),
    })
})
    .then(response => {
        if (!response.ok) {
            throw new Error('Invalid session token');
        }
        return response.json();
    })
    .then(data => {
        sessionStorage.setItem('name', data?.name);
    })
    .catch(_ => {
        sessionStorage.removeItem("token")
        sessionStorage.removeItem("id")
        window.location.href = '/login';
    });