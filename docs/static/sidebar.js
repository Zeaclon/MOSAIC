document.addEventListener("DOMContentLoaded", () => {
    const toggles = document.querySelectorAll(".sidebar-toggle");

    toggles.forEach((toggle) => {
        const section = toggle.closest("li");
        const pages = section.querySelector(".sidebar-pages");
        const arrow = toggle.querySelector("span");

        if (!pages) {
            return;
        }

        const expanded = toggle.getAttribute("aria-expanded") === "true";

        if (expanded) {
            pages.style.display = "block";
            arrow.style.transform = "rotate(90deg)";
        } else {
            pages.style.display = "none";
            arrow.style.transform = "rotate(0deg)";
        }

        toggle.addEventListener("click", () => {
            const isExpanded =
                toggle.getAttribute("aria-expanded") === "true";

            toggle.setAttribute(
                "aria-expanded",
                String(!isExpanded)
            );

            if (isExpanded) {
                pages.style.display = "none";
                arrow.style.transform = "rotate(0deg)";
            } else {
                pages.style.display = "block";
                arrow.style.transform = "rotate(90deg)";
            }
        });
    });
});