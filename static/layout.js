if (!getCookieValue('token') || !getCookieValue('id')) {
    logout();
}

fetch('/api/auth/me', {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json'
    },
    body: JSON.stringify({
        id: getCookieValue('id'),
        token: getCookieValue('token'),
    })
})
    .then(response => {
        if (!response.ok) {
            throw new Error('Invalid session token');
        }
        return response.json();
    })
    .then(data => {
        document.cookie = `name=${data.name}; path=/; SameSite=Strict;`;
    })
    .catch(_ => logout());

function logout() {
    fetch('/api/auth/logout', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            id: getCookieValue('id'),
            token: getCookieValue('token'),
        })
    }).catch(_ => {
        document.cookie = "token=; path=/; expires=Thu, 01 Jan 1970 00:00:00 UTC;";
        document.cookie = "id=; path=/; expires=Thu, 01 Jan 1970 00:00:00 UTC;";
        document.cookie = "name=; path=/; expires=Thu, 01 Jan 1970 00:00:00 UTC;";
        window.location.href = '/login';
    })
}