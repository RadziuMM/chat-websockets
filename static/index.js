if (!sessionStorage.getItem('token')) {
    window.location.href = '/login';
}

console.log("Hello world!")