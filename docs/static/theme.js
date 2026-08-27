document.addEventListener("DOMContentLoaded", () => {
    const selector = document.getElementById("theme-selector");

    if (!selector) {
        return;
    }

    const savedTheme = localStorage.getItem("mosaic-theme");

    if (savedTheme === "light" || savedTheme === "dark") {
        document.documentElement.dataset.theme = savedTheme;
        selector.value = savedTheme;
    } else {
        selector.value = "system";
    }

    selector.addEventListener("change", () => {
        const theme = selector.value;

        if (theme === "system") {
            document.documentElement.removeAttribute("data-theme");
            localStorage.removeItem("mosaic-theme");
            return;
        }

        document.documentElement.dataset.theme = theme;
        localStorage.setItem("mosaic-theme", theme);
    });
});