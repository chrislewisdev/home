function changeTheme(theme) {
    document.body.className = theme;

    var switchers = document.getElementsByClassName('theme-switcher');
    Array.prototype.forEach.call(switchers, e => e.removeAttribute('disabled'));

    document.getElementById(`theme-switch-${theme}`).setAttribute('disabled', 'true');

    localStorage.setItem('theme', theme);
}

document.addEventListener("DOMContentLoaded", () => {
    var theme = localStorage.getItem('theme');
    if (theme) {
        changeTheme(theme);
    } else {
        changeTheme('palenight');
    }
});
